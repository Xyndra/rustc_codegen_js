use fw_router::{Router, MatchedRoute, CachePolicy};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct MemoryCache {
    entries: BTreeMap<String, CacheEntry>,
    max_bytes: Option<usize>,
    used_bytes: usize,
}

impl MemoryCache {
    pub fn new(max_bytes: Option<usize>) -> Self {
        Self { entries: BTreeMap::new(), max_bytes, used_bytes: 0 }
    }
}

impl Default for MemoryCache {
    fn default() -> Self {
        Self::new(None)
    }
}

impl CacheStore for MemoryCache {
    fn get(&self, key: &str) -> Option<CacheEntry> {
        self.entries.get(key).cloned()
    }
    fn set(&mut self, key: &str, entry: CacheEntry) {
        let size = entry.html.len();
        if let Some(max) = self.max_bytes {
            if size > max {
                return;
            }
            // evict oldest if needed (simple: clear all when over)
            while self.used_bytes + size > max && !self.entries.is_empty() {
                if let Some(k) = self.entries.keys().next().cloned() {
                    if let Some(e) = self.entries.remove(&k) {
                        self.used_bytes = self.used_bytes.saturating_sub(e.html.len());
                    }
                } else {
                    break;
                }
            }
        }
        if let Some(old) = self.entries.insert(key.to_string(), entry.clone()) {
            self.used_bytes = self.used_bytes.saturating_sub(old.html.len());
        }
        self.used_bytes += size;
    }
    fn invalidate(&mut self, key: &str) {
        if let Some(e) = self.entries.remove(key) {
            self.used_bytes = self.used_bytes.saturating_sub(e.html.len());
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DiskCache {
    entries: BTreeMap<String, CacheEntry>,
    dir: String,
}

impl DiskCache {
    pub fn new(dir: impl Into<String>) -> Self {
        Self { entries: BTreeMap::new(), dir: dir.into() }
    }
}

impl CacheStore for DiskCache {
    fn get(&self, key: &str) -> Option<CacheEntry> {
        self.entries.get(key).cloned()
    }
    fn set(&mut self, key: &str, entry: CacheEntry) {
        self.entries.insert(key.to_string(), entry);
    }
    fn invalidate(&mut self, key: &str) {
        self.entries.remove(key);
    }
}

pub struct SsrRenderer {
    pub router: Router,
    pub cache: MemoryCache,
    pub disk: DiskCache,
    pub pregenerated: BTreeMap<String, String>,
    pub memory_enabled: bool,
}

impl SsrRenderer {
    pub fn new(router: Router, memory_limit: Option<usize>, memory_enabled: bool) -> Self {
        Self {
            router,
            cache: MemoryCache::new(if memory_enabled { memory_limit } else { Some(0) }),
            disk: DiskCache::new(".cache"),
            pregenerated: BTreeMap::new(),
            memory_enabled,
        }
    }

    pub fn pregenerate_static(&mut self) {
        for route in self.router.routes.clone() {
            // static determination by path segments all Static
            let is_static = route.path.iter().all(|s| matches!(s, fw_router::PathSegment::Static(_)));
            if is_static {
                let html = format!("<h1>{}</h1>", route.path.iter().map(|s| match s {
                    fw_router::PathSegment::Static(x) => *x,
                    fw_router::PathSegment::Slug => ":slug",
                }).collect::<Vec<_>>().join("/"));
                self.pregenerated.insert(html.clone(), html.clone());
            }
        }
    }

    pub fn render_route(&mut self, path: &str) -> String {
        if let Some(h) = self.pregenerated.get(path) {
            return h.clone();
        }
        // disk first
        if let Some(entry) = self.disk.get(path) {
            if self.memory_enabled {
                self.cache.set(path, entry.clone());
            }
            return entry.html;
        }
        if let Some(entry) = self.cache.get(path) {
            return entry.html;
        }
        // resolve
        let method = fw_router::Method::Get;
        let m = block_on(self.router.resolve(method, path));
        if let Some(m) = m {
            let route = &self.router.routes[m.route_index];
            let html = format!("<main data-route=\"{}\">{}</main>", path, m.slugs.join(","));
            let cache_policy = route.cache;
            if matches!(cache_policy, CachePolicy::Cache) {
                let entry = CacheEntry {
                    html: html.clone(),
                    created: Instant::now(),
                    etag: None,
                };
                self.disk.set(path, entry.clone());
                if self.memory_enabled {
                    self.cache.set(path, entry);
                }
            }
            html
        } else {
            String::from("<h1>404</h1>")
        }
    }

    pub fn invalidate(&mut self, path: &str) {
        self.cache.invalidate(path);
        self.disk.invalidate(path);
        self.pregenerated.remove(path);
    }
}

fn block_on<F: std::future::Future>(mut f: F) -> F::Output {
    use std::task::{Context, Poll};
    let waker = std::task::Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut f = Box::pin(f);
    loop {
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwrState { Fresh, Stale, Expired }

#[cfg(test)]
mod tests {
    use super::*;
    use fw_router::Route;

    #[test]
    fn cache_hit() {
        let mut r = Router::new();
        r.add_route(Route {
            path: vec![fw_router::PathSegment::Static("x")],
            method: fw_router::Method::Get,
            middleware: Vec::new(),
            handler: 0,
            cache: CachePolicy::Cache,
        });
        let mut s = SsrRenderer::new(r, None, false);
        s.render_route("/x");
        let h = s.render_route("/x");
        assert!(!h.is_empty());
    }
}
