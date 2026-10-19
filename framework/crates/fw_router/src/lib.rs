pub use inventory;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachePolicy {
    None,
    Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl Method {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(Method::Get),
            "POST" => Some(Method::Post),
            "PUT" => Some(Method::Put),
            "DELETE" => Some(Method::Delete),
            "PATCH" => Some(Method::Patch),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathSegment {
    Static(&'static str),
    Slug,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub path: Vec<PathSegment>,
    pub method: Method,
    pub middleware: Vec<usize>,
    pub handler: usize,
    pub cache: CachePolicy,
}

pub struct Router {
    pub routes: Vec<Route>,
}

impl Default for Router {
    fn default() -> Self { Self::new() }
}

impl Router {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    pub fn add(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn add_route(&mut self, route: Route) {
        self.add(route);
    }

    pub async fn resolve(&self, method: Method, path: &str) -> Option<MatchedRoute> {
        let decoded = percent_decode(path);
        let segments: Vec<&str> = decoded.split('/').filter(|s| !s.is_empty()).collect();
        for (i, route) in self.routes.iter().enumerate() {
            if route.method != method {
                continue;
            }
            if route.path.len() != segments.len() {
                continue;
            }
            let mut slugs = Vec::new();
            let mut ok = true;
            for (seg, pat) in segments.iter().zip(route.path.iter()) {
                match pat {
                    PathSegment::Static(s) => {
                        if *s != *seg {
                            ok = false;
                            break;
                        }
                    }
                    PathSegment::Slug => {
                        slugs.push(percent_decode(seg));
                    }
                }
            }
            if ok {
                return Some(MatchedRoute {
                    route_index: i,
                    slugs,
                });
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct MatchedRoute {
    pub route_index: usize,
    pub slugs: Vec<String>,
}

pub type Params = MatchedRoute;

#[derive(Debug, Clone, Default)]
pub struct Request {
    pub method: Option<Method>,
    pub path: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct Response {
    pub status: u16,
    pub body: String,
    pub headers: BTreeMap<String, String>,
}

impl Response {
    pub fn new(status: u16) -> Self {
        Self {
            status,
            ..Self::default()
        }
    }
    pub fn ok() -> Self {
        Self {
            status: 200,
            ..Self::default()
        }
    }
}

pub trait Middleware {
    fn before(&self, req: &Request) -> Result<(), Response>;
    fn after(&self, _req: &Request, _res: &mut Response) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_match() {
        let mut r = Router::new();
        r.add(Route {
            path: vec![PathSegment::Static("blog"), PathSegment::Slug],
            method: Method::Get,
            middleware: Vec::new(),
            handler: 0,
            cache: CachePolicy::None,
        });
        let m = futures_block_on(r.resolve(Method::Get, "/blog/hello")).unwrap();
        assert_eq!(m.slugs, vec!["hello".to_string()]);
    }

    #[test]
    fn static_match() {
        let mut r = Router::new();
        r.add(Route {
            path: vec![PathSegment::Static("about")],
            method: Method::Get,
            middleware: Vec::new(),
            handler: 0,
            cache: CachePolicy::None,
        });
        assert!(futures_block_on(r.resolve(Method::Get, "/about")).is_some());
        assert!(futures_block_on(r.resolve(Method::Get, "/nope")).is_none());
    }
}

// helper for tests (sync resolve wrapper)
fn futures_block_on<F: std::future::Future>(f: F) -> F::Output {
    // simple block on using a tiny executor not available; use poll
    use std::task::{Context, Poll};
    let mut f = Box::pin(f);
    let waker = std::task::Waker::noop();
    let mut cx = Context::from_waker(waker);
    loop {
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}


#[derive(Debug)]
pub struct RegisteredRoute {
    pub path: &'static str,
    pub method: Method,
    pub cache: CachePolicy,
}

inventory::collect!(RegisteredRoute);

pub fn path_segments(path: &'static str) -> Vec<PathSegment> {
    path.split('/').filter(|s| !s.is_empty()).map(|seg| {
        if seg.starts_with(':') {
            PathSegment::Slug
        } else {
            PathSegment::Static(leak(seg))
        }
    }).collect()
}

fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

pub fn build_router() -> Router {
    let mut r = Router::new();
    for reg in inventory::iter::<RegisteredRoute> {
        r.add(Route {
            path: path_segments(reg.path),
            method: reg.method,
            middleware: Vec::new(),
            handler: 0,
            cache: reg.cache,
        });
    }
    r
}


fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hex = &input[i+1..i+3];
            if let Ok(val) = u8::from_str_radix(hex, 16) {
                out.push(val as char);
                i += 3;
                continue;
            }
        }
        if bytes[i] == b'+' {
            out.push(' ');
        } else {
            out.push(bytes[i] as char);
        }
        i += 1;
    }
    out
}
