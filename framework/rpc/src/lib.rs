use fw_router::Method;
use fw_server::{Request, Response, Server};

pub struct RpcEndpoint {
    pub method: Method,
    pub path: &'static str,
    pub handler: fn(Request) -> Response,
}

pub trait RpcClient {
    async fn call(&self, method: Method, path: &str, body: &[u8]) -> anyhow::Result<Vec<u8>>;
}

#[derive(Debug)]
pub enum RpcError {
    Transport(String),
    Status(u16, String),
    Parse(String),
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::Transport(s) => write!(f, "transport: {}", s),
            RpcError::Status(c, s) => write!(f, "status {}: {}", c, s),
            RpcError::Parse(s) => write!(f, "parse: {}", s),
        }
    }
}
impl std::error::Error for RpcError {}

pub struct RpcServer {
    pub router: fw_router::Router,
    pub endpoints: Vec<RpcEndpoint>,
    pub prefix: String,
}

impl RpcServer {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self { router: fw_router::Router::new(), endpoints: Vec::new(), prefix: prefix.into() }
    }

    pub fn register(&mut self, ep: RpcEndpoint) {
        let path = format!("{}{}", self.prefix, ep.path);
        let segments: Vec<fw_router::PathSegment> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| fw_router::PathSegment::Static(leak(s)))
            .collect();
        self.router.add(fw_router::Route {
            path: segments,
            method: ep.method,
            middleware: Vec::new(),
            handler: 0,
            cache: fw_router::CachePolicy::None,
        });
        self.endpoints.push(ep);
    }

    pub fn handle(&self, req: Request) -> Response {
        for ep in &self.endpoints {
            let full = format!("{}{}", self.prefix, ep.path);
            if full == req.path {
                let m = ep.method;
                let rm = req.method.unwrap_or(m);
                if m == rm {
                    return (ep.handler)(req);
                }
            }
        }
        Response::not_found()
    }

    pub fn mount_into<S: Server>(&self, server: &mut S) {
        for ep in &self.endpoints {
            let full = format!("{}{}", self.prefix, ep.path);
            let segments: Vec<fw_router::PathSegment> = full
                .split('/')
                .filter(|s| !s.is_empty())
                .map(|s| fw_router::PathSegment::Static(leak(s)))
                .collect();
            server.router().add(fw_router::Route {
                path: segments,
                method: ep.method,
                middleware: Vec::new(),
                handler: 0,
                cache: fw_router::CachePolicy::None,
            });
        }
    }
}

fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

pub struct HttpRpcClient {
    pub base_url: String,
}

impl HttpRpcClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into() }
    }
}

impl RpcClient for HttpRpcClient {
    async fn call(&self, method: Method, path: &str, body: &[u8]) -> anyhow::Result<Vec<u8>> {
        use std::io::{Read, Write};
        use std::net::TcpStream;
        let url = self.base_url.trim_end_matches('/');
        let hostpart = url.strip_prefix("http://").unwrap_or(url);
        let hostport = hostpart.split('/').next().unwrap_or(hostpart);
        let (host, port) = if let Some((h, p)) = hostport.split_once(':') {
            (h, p.parse::<u16>().unwrap_or(80))
        } else {
            (hostport, 80u16)
        };
        let mut stream = TcpStream::connect((host, port)).map_err(|e| anyhow::anyhow!(e))?;
        let req = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            method.as_str(), path, hostport, body.len()
        );
        stream.write_all(req.as_bytes()).map_err(|e| anyhow::anyhow!(e))?;
        stream.write_all(body).map_err(|e| anyhow::anyhow!(e))?;
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).map_err(|e| anyhow::anyhow!(e))?;
        let text = String::from_utf8_lossy(&buf);
        let body_start = text.find("\r\n\r\n").map(|i| i + 4).unwrap_or(buf.len());
        let status_line = text.lines().next().unwrap_or("");
        let status_code: u16 = status_line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        let body_bytes = buf[body_start..].to_vec();
        if status_code >= 400 {
            return Err(anyhow::anyhow!("status {}", status_code));
        }
        Ok(body_bytes)
    }
}

pub fn rpc_handler_example(req: Request) -> Response {
    Response::json(200, format!("{{\"echo\":\"{}\"}}", String::from_utf8_lossy(&req.body)))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_handle() {
        let mut srv = RpcServer::new("/api");
        srv.register(RpcEndpoint {
            method: Method::Get,
            path: "/hello",
            handler: |_| Response::json(200, "{\"ok\":true}"),
        });
        let req = Request { method: Some(Method::Get), path: "/api/hello".into(), headers: Vec::new(), body: Vec::new() };
        let resp = srv.handle(req);
        assert_eq!(resp.status, 200);
    }

    #[test]
    fn not_found() {
        let srv = RpcServer::new("/api");
        let req = Request { method: Some(Method::Get), path: "/api/nope".into(), headers: Vec::new(), body: Vec::new() };
        let resp = srv.handle(req);
        assert_eq!(resp.status, 404);
    }
}
