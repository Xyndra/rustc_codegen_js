# Design Specification: Rust Web Framework on rustc_codegen_js

## 1. Goals

- Single Rust component source compiled twice: to JavaScript (client bundle) via
  `rustc_codegen_js`, and to native code (server) via normal `rustc`.
- Manual dirty-flagging with partial rerender.
- Rendering is string-based; native (future) parses HTML strings.
- Abstract server: io_uring specialization on Linux, portable fallback elsewhere.
- SSR with caching policies; static route pregeneration.
- Tailwind via bun/tailwind CLI once per project; Rust-side CSS subset per page.
- HTTP RPC/REST to call server code.

## 2. Workspace Layout

```
framework/
  Cargo.toml                 # workspace
  DESIGN.md
  core/                      # traits, nodes, dirty flags, scheduler, styles
  render/                    # string mounting/hydration helpers
  router/                    # routing, params, middleware
  server/                    # Server trait + io_uring/fallback
  ssr/                       # SSR orchestration + caches
  rpc/                       # RPC/REST client+server glue
  tailwind/                  # style manifest + runtime dynamic loader
  framework/                 # facade crate + app builder + prelude
```


## 3. Core Data Structures

```rust
pub type ComponentId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirtyMask { None, SelfOnly, Subtree }

pub struct ComponentState {
    pub id: ComponentId,
    pub dirty: DirtyMask,
    pub parent: Option<ComponentId>,
    pub children: Vec<ComponentId>,
}
```

Components store their own children and properties as ordinary struct fields. The trait exposes dirty control (usable from outside and inside), a render that returns an HTML string (built via a macro), and the set of style classes that may apply to itself and its children:

```rust
pub trait Component {
    // dirty control
    fn set_dirty(&mut self, mask: DirtyMask);
    fn is_dirty(&self) -> DirtyMask;

    // render produces the component's HTML string (macro-assisted)
    fn render(&mut self, cx: &mut RenderCx) -> String;

    // CSS classes that may apply to this component and its children
    fn possible_css_classes(&self) -> &'static str;
}
```

Dirty helpers have default implementations where possible; properties are not abstracted by the trait and are handled individually by each component struct (with default impls for common helpers).

Data flow between parent/children (and across elements) uses a boxed setter that writes into a local field and marks the owning component dirty:

```rust
pub type Setter<T> = Box<dyn FnMut(T)>;
// invoking a setter writes the value to its captured field and calls set_dirty
```

## 4. Rendering Model

Rendering is string-based. A component's `render` returns an HTML string (built with a
macro). There is no `RenderTarget` trait and no per-node handles:

- Client: mount/patch a component's string into the DOM (innerHTML / targeted replacement).
- SSR: return the rendered string directly.
- Native (future): parse the HTML string with a lightweight XML/HTML parser, like a browser.

Hydration on the client reuses server-rendered DOM by matching component boundaries.

## 5. Dirty Flagging Algorithm

1. A component calls `set_dirty(mask)` (available from outside and inside):
   - store `dirty = mask`.
   - insert its id into `RenderScheduler::pending`.
   - rendering is always batched; no immediate flush occurs here.

   Setters created via the boxed `Setter<T>` also call `set_dirty` after writing.

2. `flush()` (single pass, batched):
   - set `rendering = true`.
   - process pending ids in top-down tree order.
   - for each dirty component, re-run `render()`, producing a new HTML string.
   - compare against the previously stored string; if different, replace that
     component's rendered region in the DOM with the new string.
   - clear dirty flags for rendered components; remove from pending.
   - set `rendering = false`.

3. Scheduling:
   - Pending accumulates across events/handlers.
   - A single `flush()` runs at end of full render (`end_render()`) or at an
     explicit flush point.
   - Events call `set_dirty`; flushing is batched, never immediate.

4. Non-dirty subtrees are skipped entirely (no render, no DOM writes).

## 6. Lifecycle

Client:
1. Load JS bundle (compiled components + runtime).
2. Hydrate server-rendered DOM (reuse nodes, attach handlers, dirty=false).
3. User events -> mark_dirty -> scheduler flush.
4. On navigation/route change -> render new route component.

Server:
1. Boot: build router, warm static caches.
2. Accept connections via `Server` impl.
3. Request -> middleware chain -> route handler -> (SSR render if page) -> response.
4. Shutdown: flush caches to disk if configured.

Component lifecycle hooks (optional): `on_mount`, `on_update`, `on_unmount`.

## 7. Routing

```rust
pub enum Method { Get, Post, Put, Delete, Patch }

pub enum PathSegment {
    Static(&'static str),
    Slug, // unnamed slug segment
}

pub struct Route {
    pub path: Vec<PathSegment>,
    pub method: Method,
    pub middleware: Vec<MiddlewareId>,
    pub handler: Handler,
}

pub struct Router {
    pub routes: Vec<Route>,
}

pub struct Params { pub slugs: Vec<String> }

impl Router {
    pub fn add(&mut self, route: Route);
    pub async fn resolve(&self, method: Method, path: &str) -> Option<(Route, Params)>;
}
```

All routes live in one list; resolution matches both method and path, so the same path with
different methods does not overwrite. Slugs carry no parameter name; matched slug segments
are returned as an ordered `Vec<String>`. Middleware:

```rust
pub trait Middleware {
    async fn call(&self, req: Request, next: Next) -> Response;
}
```

API endpoints registered as routes (REST) or under an RPC prefix.

## 8. Server Trait

```rust
pub trait Server {
    type Error = anyhow::Error;
    async fn bind(&mut self, addr: SocketAddr) -> anyhow::Result<()>;
    async fn serve(&self) -> anyhow::Result<()>;
    fn router(&mut self) -> &mut Router;
    fn add_middleware(&mut self, mw: Box<dyn Middleware>);
}

pub fn make_server() -> Box<dyn Server> {
    #[cfg(all(target_os = "linux", feature = "io_uring"))]
    { Box::new(IoUringServer::new()) }
    #[cfg(not(all(target_os = "linux", feature = "io_uring")))]
    { Box::new(FallbackServer::new()) }
}
```

- `IoUringServer`: Linux io_uring based accept/read/write.
- `FallbackServer`: portable async runtime (std thread pool or tokio if enabled).

Router resolution is asynchronous: `Router::resolve` is `async fn`.

## 9. SSR Flow

1. Server receives page request.
2. Router resolves route (async).
3. If static route and cache hit, return cached HTML.
4. Else render:
   - instantiate route component with props from params (slugs).
   - render component to an HTML string.
   - collect style classes from the default (prerendered) render state; these are
     required, not optional, and are written directly into the served HTML document
     (inline) so the page is never unstyled.
5. Apply cache policy; store result.
6. Return HTML response with hydration marker, critical styles inlined, and
   references to load the JS bundle plus the remaining style bundle. The remaining
   styles are the difference between the component's `possible_css_classes` and the
   classes already inlined with the prerender.

Hydration on client: client bundle reuses server DOM, attaches handlers, sets dirty=false.

## 10. Caching

```rust
pub enum CachePolicy {
    None,   // no caching (e.g. user-data routes)
    Cache,  // disk-cache always; memory-cache optional
}

pub struct CacheEntry {
    pub html: String,
    pub created: Instant,
    pub etag: Option<String>,
}

pub trait CacheStore {
    fn get(&self, key: &str) -> Option<CacheEntry>;
    fn set(&mut self, key: &str, entry: CacheEntry);
    fn invalidate(&mut self, key: &str);
}
```

Implementations: `DiskCache` (always, persistent) and `MemoryCache` (optional, with a configurable
RAM cap; entries are promoted from disk only when memory caching is enabled). Static routes are
pregenerated at build/deploy time into cache. Slug routes use the configured `CachePolicy` at
runtime.

Stale-while-revalidate is not used; caching is binary (cache or not).

Static site generation: the framework can emit, for every route, a standalone HTML + JS + CSS
file set suitable for static hosting (no server required).

## 11. Tailwind / CSS Flow

1. Every component implements `possible_css_classes` (merged into `Component`), listing the
   CSS classes that may apply to itself and its children.

2. Tailwind runs once per project (not per page/site) and generates a bundle containing
   ALL classes.

3. `lightningcss` (Rust) optimizes that bundle into a single `.min.css`.

4. The framework uses a lightningcss-like CSS analysis tool to determine which rules are
   related to the classes actually used by a page and ships only those (per-page subset).

## 12. RPC / REST

```rust
pub struct RpcEndpoint {
    pub method: Method,
    pub path: &'static str,
    pub handler: fn(Request) -> Response,
}

pub trait RpcClient {
    async fn call(&self, method: Method, path: &str, body: &[u8]) -> anyhow::Result<Vec<u8>>;
}
```

Server registers RPC endpoints as routes under `/api` (or configured prefix). Client calls via HTTP. Serialization left to user (serde recommended). Middleware applies to RPC routes too.

## 13. Proc Macros

- `#[component]`: applied to a component struct/impl. Generates `possible_css_classes`
  from every `html!()` invocation in the implementation and injects required fields
  (children, and possibly an id) into the struct.

- `#[page]`, `#[rpc]`, `#[rest]`: applied to route handlers/endpoints. Register the
  route and its bundle metadata.

- Collected routes are gathered into a `router!()` invocation in the crate root (main
  file) using a `macro_magic`-style cross-macro collection mechanism, so the app's
  router is assembled automatically.

## 14. Open Items

- Exception capture in DOM bindings.
- Native HTML parsing/rendering.
- io_uring implementation details.
