pub use fw_core::*;
pub use fw_render::*;
pub use fw_router::{CachePolicy, MatchedRoute, PathSegment, Route, Router};
pub use fw_server::*;
pub use fw_ssr::*;
pub use fw_rpc::*;
pub use fw_tailwind::*;

pub mod prelude {
    pub use crate::{
        ApiHandler, CacheEntry, CachePolicy, CacheStore, DiskCache, MatchedRoute, MemoryCache,
        Method, Middleware, PageRenderer, PathSegment, Request, Response, Route, Router, RpcClient,
        RpcEndpoint, RpcError, RpcServer, Server, SsrRenderer, SwrState,
    };
    pub use crate::{ApiChain, PageShell, StaticFiles, run_site, run_site_at};
    pub use fw_core::prelude::*;
    pub use fw_render::*;
    pub use fw_tailwind::{DynamicStyleRegistry, StyleRegistry};
}

pub struct App {
    pub router: Router,
}

impl App {
    pub fn new() -> Self {
        Self { router: Router::new() }
    }
    pub fn router_mut(&mut self) -> &mut Router {
        &mut self.router
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "macros")]
pub use fw_macros::*;

#[macro_export]
macro_rules! html {
    ($s:expr) => {
        $s.to_string()
    };
}

pub struct StaticFiles {
    pub dir: String,
}

impl StaticFiles {
    pub fn new(dir: impl Into<String>) -> Self {
        Self { dir: dir.into() }
    }
}

impl fw_server::ApiHandler for StaticFiles {
    fn handle_api(&self, req: &fw_server::Request) -> Option<fw_server::Response> {
        if req.method != Some(fw_router::Method::Get) {
            return None;
        }
        let name = req.path.trim_start_matches('/');
        if name.ends_with(".mjs") || name == "assets/tailwind.css" {
            let file = if name == "assets/tailwind.css" { "tailwind-lazy.css".to_string() } else { name.to_string() };
            let path = format!("{}/{}", self.dir, file);
            if let Ok(content) = std::fs::read_to_string(&path) {
                let mut r = fw_server::Response::new(200, content.into_bytes());
                if name.ends_with(".mjs") {
                    r.headers.push(("Content-Type".into(), "text/javascript".into()));
                } else {
                    r.headers.push(("Content-Type".into(), "text/css".into()));
                }
                return Some(r);
            }
        }
        None
    }
}

pub struct FnPageRenderer<F>(pub F);

impl<F> fw_server::PageRenderer for FnPageRenderer<F>
where
    F: Fn(&str) -> String + Send + Sync,
{
    fn render_page(&self, path: &str, _slugs: &[String]) -> String {
        (self.0)(path)
    }
    fn render_not_found(&self) -> String {
        (self.0)("/404")
    }
}

pub struct ApiChain {
    handlers: Vec<std::sync::Arc<dyn fw_server::ApiHandler>>,
}

impl ApiChain {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    pub fn with(mut self, handler: impl fw_server::ApiHandler + 'static) -> Self {
        self.handlers.push(std::sync::Arc::new(handler));
        self
    }
}

impl Default for ApiChain {
    fn default() -> Self {
        Self::new()
    }
}

impl fw_server::ApiHandler for ApiChain {
    fn handle_api(&self, req: &fw_server::Request) -> Option<fw_server::Response> {
        self.handlers.iter().find_map(|h| h.handle_api(req))
    }
}

pub struct PageShell {
    template: &'static str,
    critical: String,
}

impl PageShell {
    pub fn new(template: &'static str, critical: impl Into<String>) -> Self {
        Self { template, critical: critical.into() }
    }

    /// Loads the critical CSS that `cargo site` wrote next to the page bundles.
    pub fn from_bundles(template: &'static str) -> Self {
        let dir = std::env::var("BUNDLES_DIR").unwrap_or_else(|_| "bundles".to_string());
        let critical = std::fs::read_to_string(std::path::Path::new(&dir).join("critical.css"))
            .unwrap_or_default();
        Self::new(template, critical)
    }

    pub fn render(&self, body: &str, bundle: &str) -> String {
        let script = format!(
            r#"<script>
document.addEventListener("DOMContentLoaded", () => {{
  const css = document.createElement("link");
  css.rel = "stylesheet";
  css.href = "/assets/tailwind.css";
  const preload = document.createElement("link");
  preload.rel = "modulepreload";
  preload.href = "/{bundle}.mjs";
  document.head.append(css, preload);
  const boot = () => {{
    document.documentElement.dataset.lazyCss = "ready";
    const script = document.createElement("script");
    script.type = "module";
    script.src = "/{bundle}.mjs";
    document.body.append(script);
  }};
  css.addEventListener("load", boot, {{ once: true }});
  css.addEventListener("error", boot, {{ once: true }});
}});
</script>"#
        );
        self.template
            .replace("/*BODY*/", body)
            .replace("/*CRITICAL*/", &self.critical)
            .replace("/*BUNDLE*/", &script)
    }
}

pub fn run_site(
    router: Router,
    renderer: impl fw_server::PageRenderer + 'static,
    api: impl fw_server::ApiHandler + 'static,
) -> anyhow::Result<()> {
    let addr = std::env::args()
        .nth(1)
        .or_else(|| std::env::var("SITE_ADDR").ok())
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    run_site_at(router, renderer, api, addr)
}

pub fn run_site_at(
    router: Router,
    renderer: impl fw_server::PageRenderer + 'static,
    api: impl fw_server::ApiHandler + 'static,
    addr: impl Into<String>,
) -> anyhow::Result<()> {
    let addr = addr.into();
    let bundles = std::env::var("BUNDLES_DIR").unwrap_or_else(|_| "bundles".to_string());
    let mut server = fw_server::make_server();
    server.bind(&addr)?;
    for route in router.routes {
        server.router().add(route);
    }
    server.set_page_renderer(std::sync::Arc::new(renderer));
    let chain = ApiChain::new()
        .with(api)
        .with(StaticFiles::new(bundles));
    server.set_api_handler(std::sync::Arc::new(chain));
    println!("listening on http://{addr}");
    server.serve()
}
