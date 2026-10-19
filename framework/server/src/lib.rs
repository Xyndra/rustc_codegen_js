use anyhow::Result;
use fw_router::Router;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub type Method = fw_router::Method;

#[derive(Clone, Debug)]
pub struct Request {
    pub method: Option<Method>,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn ok() -> Self {
        Self { status: 200, headers: Vec::new(), body: Vec::new() }
    }
    pub fn new(status: u16, body: Vec<u8>) -> Self {
        Self { status, headers: Vec::new(), body }
    }
    pub fn text(status: u16, body: impl Into<String>) -> Self {
        let mut r = Self::new(status, body.into().into_bytes());
        r.headers.push(("Content-Type".into(), "text/plain; charset=utf-8".into()));
        r
    }
    pub fn html(status: u16, body: impl Into<String>) -> Self {
        let mut r = Self::new(status, body.into().into_bytes());
        r.headers.push(("Content-Type".into(), "text/html; charset=utf-8".into()));
        r
    }
    pub fn json(status: u16, body: impl Into<String>) -> Self {
        let mut r = Self::new(status, body.into().into_bytes());
        r.headers.push(("Content-Type".into(), "application/json".into()));
        r
    }
    pub fn not_found() -> Self {
        Self::text(404, "Not Found")
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let reason = match self.status {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "OK",
        };
        let _ = write!(out, "HTTP/1.1 {} {}\r\n", self.status, reason);
        let mut headers = self.headers.clone();
        if !headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("Content-Length")) {
            headers.push(("Content-Length".into(), self.body.len().to_string()));
        }
        if !headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("Connection")) {
            headers.push(("Connection".into(), "close".into()));
        }
        for (k, v) in &headers {
            let _ = write!(out, "{}: {}\r\n", k, v);
        }
        out.extend_from_slice(b"\r\n");
        out.extend_from_slice(&self.body);
        out
    }
}

pub trait Middleware: Send + Sync {
    fn before(&self, req: &mut Request) -> Result<(), Response> {
        let _ = req;
        Ok(())
    }
    fn after(&self, _req: &Request, _res: &mut Response) {}
}

pub trait PageRenderer: Send + Sync {
    fn render_page(&self, path: &str, slugs: &[String]) -> String;
    fn render_not_found(&self) -> String { String::from("Not Found") }
}

pub trait ApiHandler: Send + Sync {
    fn handle_api(&self, req: &Request) -> Option<Response>;
}

#[derive(Default)]
pub struct Core {
    pub router: Router,
    pub middleware: Vec<Box<dyn Middleware>>,
    pub page_renderer: Option<Arc<dyn PageRenderer>>,
    pub api_handler: Option<Arc<dyn ApiHandler>>,
}

impl Core {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dispatch(&self, req: Request) -> Response {
        let mut req = req;
        for mw in &self.middleware {
            if let Err(resp) = mw.before(&mut req) {
                return resp;
            }
        }
        let mut resp = self.route(&req);
        for mw in self.middleware.iter().rev() {
            mw.after(&req, &mut resp);
        }
        // content encoding: try accepted encodings, pick smallest output
        let accept = req
            .headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("Accept-Encoding"))
            .map(|(_, v)| v.clone())
            .unwrap_or_default();
        let already_encoded = resp.headers.iter().any(|(k, _)| k.eq_ignore_ascii_case("Content-Encoding"));
        if !already_encoded && resp.body.len() >= 200 && !accept.is_empty() {
            let is_bundle = req.path.ends_with(".mjs") || req.path.ends_with(".css");
            // bundles prefer br if accepted
            if is_bundle && accept.contains("br") {
                // handled by Api precompressed; skip runtime
            }
            // zstd
            if accept.contains("zstd") {
                use std::io::Write;
                if let Ok(mut enc) = zstd::Encoder::new(Vec::new(), 0) {
                    let _ = enc.write_all(&resp.body);
                    if let Ok(c) = enc.finish() {
                        resp.body = c;
                        resp.headers.push(("Content-Encoding".into(), "zstd".into()));
                    }
                }
            } else if accept.contains("gzip") {
                use flate2::write::GzEncoder;
                use flate2::Compression;
                use std::io::Write;
                let mut enc = GzEncoder::new(Vec::new(), Compression::default());
                let _ = enc.write_all(&resp.body);
                if let Ok(c) = enc.finish() {
                    resp.body = c;
                    resp.headers.push(("Content-Encoding".into(), "gzip".into()));
                }
            } else if accept.contains("deflate") {
                use flate2::write::DeflateEncoder;
                use flate2::Compression;
                use std::io::Write;
                let mut enc = DeflateEncoder::new(Vec::new(), Compression::default());
                let _ = enc.write_all(&resp.body);
                if let Ok(c) = enc.finish() {
                    resp.body = c;
                    resp.headers.push(("Content-Encoding".into(), "deflate".into()));
                }
            }
        }
        resp
    }

    fn route(&self, req: &Request) -> Response {
        if let Some(api) = &self.api_handler {
            if let Some(resp) = api.handle_api(req) {
                return resp;
            }
        }
        let method = match req.method {
            Some(m) => m,
            None => return Response::not_found(),
        };
        let fut = self.router.resolve(method, &req.path);
        let m = block_on(fut);
        match m {
            Some(m) => {
                let _route = &self.router.routes[m.route_index];
                match &self.page_renderer {
                    Some(p) => Response::html(200, p.render_page(&req.path, &m.slugs)),
                    None => Response::html(
                        200,
                        format!("<h1>{}</h1><p>slugs={}</p>", req.path, m.slugs.len()),
                    ),
                }
            }
            None => match &self.page_renderer {
                Some(p) => Response::html(404, p.render_not_found()),
                None => Response::not_found(),
            },
        }
    }
}

pub trait Server {
    fn bind(&mut self, addr: &str) -> Result<()>;
    fn serve(&mut self) -> Result<()>;
    fn router(&mut self) -> &mut Router;
    fn add_middleware(&mut self, mw: Box<dyn Middleware>);
    fn set_page_renderer(&mut self, p: Arc<dyn PageRenderer>);
    fn set_api_handler(&mut self, h: Arc<dyn ApiHandler>);
}

pub fn make_server() -> Box<dyn Server> {
    #[cfg(all(target_os = "linux", feature = "io_uring"))]
    {
        Box::new(IoUringServer::new())
    }
    #[cfg(not(all(target_os = "linux", feature = "io_uring")))]
    {
        Box::new(FallbackServer::new())
    }
}

fn read_request(stream: &TcpStream) -> Option<Request> {
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(5)));
    let mut reader = BufReader::new(stream.try_clone().ok()?);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_err() {
        return None;
    }
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    let method = Method::parse(parts[0]);
    let path = parts[1].to_string();
    let mut headers = Vec::new();
    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).is_err() {
            break;
        }
        let t = line.trim_end_matches(['\r', '\n']);
        if t.is_empty() {
            break;
        }
        if let Some((k, v)) = t.split_once(':') {
            let k = k.trim().to_string();
            let v = v.trim().to_string();
            if k.eq_ignore_ascii_case("Content-Length") {
                content_length = v.parse().unwrap_or(0);
            }
            headers.push((k, v));
        }
    }
    let mut body = vec![0u8; content_length];
    let _ = reader.read_exact(&mut body);
    Some(Request { method, path, headers, body })
}

fn write_response(stream: &mut TcpStream, resp: &Response) {
    let bytes = resp.to_bytes();
    let _ = stream.write_all(&bytes);
    let _ = stream.flush();
}

// ---------- Fallback server (portable) ----------

pub struct FallbackServer {
    addr: Option<String>,
    core: Core,
}

impl FallbackServer {
    pub fn new() -> Self {
        Self { addr: None, core: Core::new() }
    }
}

impl Server for FallbackServer {
    fn bind(&mut self, addr: &str) -> Result<()> {
        self.addr = Some(addr.to_string());
        Ok(())
    }
    fn serve(&mut self) -> Result<()> {
        let addr = self.addr.clone().unwrap_or_else(|| "0.0.0.0:8080".to_string());
        let listener = TcpListener::bind(&addr).map_err(|e| anyhow::anyhow!(e))?;
        for stream in listener.incoming().flatten() {
            if let Some(req) = read_request(&stream) {
                let resp = self.core.dispatch(req);
                let mut s = stream;
                write_response(&mut s, &resp);
            }
        }
        Ok(())
    }
    fn router(&mut self) -> &mut Router {
        &mut self.core.router
    }
    fn add_middleware(&mut self, mw: Box<dyn Middleware>) {
        self.core.middleware.push(mw);
    }
    fn set_page_renderer(&mut self, p: Arc<dyn PageRenderer>) {
        self.core.page_renderer = Some(p);
    }
    fn set_api_handler(&mut self, h: Arc<dyn ApiHandler>) {
        self.core.api_handler = Some(h);
    }
}

impl Default for FallbackServer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------- io_uring server (Linux) ----------

#[cfg(target_os = "linux")]
pub struct IoUringServer {
    addr: Option<String>,
    core: Core,
}

#[cfg(target_os = "linux")]
impl IoUringServer {
    pub fn new() -> Self {
        Self { addr: None, core: Core::new() }
    }
}

#[cfg(target_os = "linux")]
impl Default for IoUringServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(target_os = "linux")]
impl Server for IoUringServer {
    fn bind(&mut self, addr: &str) -> Result<()> {
        self.addr = Some(addr.to_string());
        Ok(())
    }
    fn serve(&mut self) -> Result<()> {
        let addr = self.addr.clone().unwrap_or_else(|| "0.0.0.0:8080".to_string());
        if self.serve_uring(&addr).is_err() {
            let listener = TcpListener::bind(&addr).map_err(|e| anyhow::anyhow!(e))?;
            for stream in listener.incoming().flatten() {
                if let Some(req) = read_request(&stream) {
                    let resp = self.core.dispatch(req);
                    let mut s = stream;
                    write_response(&mut s, &resp);
                }
            }
        }
        Ok(())
    }
    fn router(&mut self) -> &mut Router {
        &mut self.core.router
    }
    fn add_middleware(&mut self, mw: Box<dyn Middleware>) {
        self.core.middleware.push(mw);
    }
    fn set_page_renderer(&mut self, p: Arc<dyn PageRenderer>) {
        self.core.page_renderer = Some(p);
    }
    fn set_api_handler(&mut self, h: Arc<dyn ApiHandler>) {
        self.core.api_handler = Some(h);
    }
}

#[cfg(target_os = "linux")]
impl IoUringServer {
    fn serve_uring(&mut self, addr: &str) -> Result<()> {
        use std::os::unix::io::{AsRawFd, FromRawFd};
        let listener = TcpListener::bind(addr).map_err(|e| anyhow::anyhow!(e))?;
        listener.set_nonblocking(true).map_err(|e| anyhow::anyhow!(e))?;
        let listen_fd = listener.as_raw_fd();
        let ring = IoUring::setup(32).map_err(|e| anyhow::anyhow!(e))?;
        loop {
            let mut addr_buf = [0u8; 128];
            let mut addrlen: u32 = 128;
            match ring.accept(listen_fd, &mut addr_buf, &mut addrlen) {
                Ok(fd) => {
                    let mut stream = unsafe { TcpStream::from_raw_fd(fd) };
                    if let Some(req) = read_request(&stream) {
                        let resp = self.core.dispatch(req);
                        write_response(&mut stream, &resp);
                    }
                }
                Err(_) => {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }
    }
}

// ---------- raw io_uring ----------

#[cfg(target_os = "linux")]
struct IoUring {
    fd: i32,
    sq_ring: *mut u8,
    cq_ring: *mut u8,
    sqes: *mut IoUringSqe,
    sq_ring_sz: usize,
    cq_ring_sz: usize,
    sqes_sz: usize,
    params: IoUringParams,
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct IoUringParams {
    sq_entries: u32,
    cq_entries: u32,
    flags: u32,
    sq_thread_cpu: u32,
    sq_thread_idle: u32,
    features: u32,
    wq_fd: u32,
    resv: [u32; 3],
    sq_off: IoUringSqRingOffsets,
    cq_off: IoUringCqRingOffsets,
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct IoUringSqRingOffsets {
    head: u32,
    tail: u32,
    ring_mask: u32,
    ring_entries: u32,
    flags: u32,
    dropped: u32,
    array: u32,
    resv1: u32,
    user_addr: u64,
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct IoUringCqRingOffsets {
    head: u32,
    tail: u32,
    ring_mask: u32,
    ring_entries: u32,
    overflow: u32,
    cqes: u32,
    flags: u32,
    resv1: u32,
    user_addr: u64,
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct IoUringSqe {
    opcode: u8,
    flags: u8,
    ioprio: u16,
    fd: i32,
    off: u64,
    addr: u64,
    len: u32,
    rw_flags: u32,
    user_data: u64,
    buf_index: u16,
    personality: u16,
    splice_fd_in: i32,
    _pad: [u64; 2],
}

#[cfg(target_os = "linux")]
#[repr(C)]
struct IoUringCqe {
    user_data: u64,
    res: i32,
    flags: u32,
}

#[cfg(target_os = "linux")]
const IORING_OP_ACCEPT: u8 = 13;
#[cfg(target_os = "linux")]
const IORING_ENTER_GETEVENTS: i32 = 1;
#[cfg(target_os = "linux")]
const IORING_OFF_SQ_RING: i64 = 0;
#[cfg(target_os = "linux")]
const IORING_OFF_CQ_RING: i64 = 0x8000000;
#[cfg(target_os = "linux")]
const IORING_OFF_SQES: i64 = 0x10000000;

#[cfg(target_os = "linux")]
impl IoUring {
    fn setup(entries: u32) -> Result<Self, String> {
        let mut params = unsafe { std::mem::zeroed::<IoUringParams>() };
        let fd = unsafe { libc::syscall(libc::SYS_io_uring_setup, entries, &mut params as *mut _) as i32 };
        if fd < 0 {
            return Err(format!("io_uring_setup failed: {}", std::io::Error::last_os_error()));
        }
        let sq_ring_sz = params.sq_off.array as usize + params.sq_entries as usize * 4;
        let cq_ring_sz =
            params.cq_off.cqes as usize + params.cq_entries as usize * std::mem::size_of::<IoUringCqe>();
        let sqes_sz = params.sq_entries as usize * std::mem::size_of::<IoUringSqe>();

        unsafe {
            let sq_ring = libc::mmap(
                std::ptr::null_mut(),
                sq_ring_sz,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_POPULATE,
                fd,
                IORING_OFF_SQ_RING,
            );
            if sq_ring == libc::MAP_FAILED {
                libc::close(fd);
                return Err("mmap sq ring failed".into());
            }
            let cq_ring = libc::mmap(
                std::ptr::null_mut(),
                cq_ring_sz,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_POPULATE,
                fd,
                IORING_OFF_CQ_RING,
            );
            if cq_ring == libc::MAP_FAILED {
                libc::munmap(sq_ring, sq_ring_sz);
                libc::close(fd);
                return Err("mmap cq ring failed".into());
            }
            let sqes = libc::mmap(
                std::ptr::null_mut(),
                sqes_sz,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | libc::MAP_POPULATE,
                fd,
                IORING_OFF_SQES,
            );
            if sqes == libc::MAP_FAILED {
                libc::munmap(sq_ring, sq_ring_sz);
                libc::munmap(cq_ring, cq_ring_sz);
                libc::close(fd);
                return Err("mmap sqes failed".into());
            }
            Ok(Self {
                fd,
                sq_ring: sq_ring as *mut u8,
                cq_ring: cq_ring as *mut u8,
                sqes: sqes as *mut IoUringSqe,
                sq_ring_sz,
                cq_ring_sz,
                sqes_sz,
                params,
            })
        }
    }

    fn accept(&self, listen_fd: i32, addr: &mut [u8], addrlen: &mut u32) -> Result<i32, String> {
        unsafe {
            let sq_tail = *(self.sq_ring.add(self.params.sq_off.tail as usize) as *const u32);
            let sq_mask = *(self.sq_ring.add(self.params.sq_off.ring_mask as usize) as *const u32);
            let sq_head = *(self.sq_ring.add(self.params.sq_off.head as usize) as *const u32);
            if sq_tail.wrapping_sub(sq_head) >= self.params.sq_entries {
                let _ = libc::syscall(
                    libc::SYS_io_uring_enter,
                    self.fd,
                    0i32,
                    0i32,
                    0i32,
                    std::ptr::null::<libc::sigset_t>(),
                );
            }
            let idx = sq_tail & sq_mask;
            let sqe = self.sqes.add(idx as usize);
            std::ptr::write_bytes(sqe, 0, 1);
            (*sqe).opcode = IORING_OP_ACCEPT;
            (*sqe).fd = listen_fd;
            (*sqe).addr = addr.as_mut_ptr() as u64;
            (*sqe).off = addrlen as *mut u32 as u64;
            (*sqe).user_data = 1;
            let array = self.sq_ring.add(self.params.sq_off.array as usize) as *mut u32;
            *array.add(idx as usize) = idx;
            *(self.sq_ring.add(self.params.sq_off.tail as usize) as *mut u32) = sq_tail.wrapping_add(1);

            let ret = libc::syscall(
                libc::SYS_io_uring_enter,
                self.fd,
                1i32,
                1i32,
                IORING_ENTER_GETEVENTS,
                std::ptr::null::<libc::sigset_t>(),
            ) as i32;
            if ret < 0 {
                return Err(format!(
                    "io_uring_enter failed: {}",
                    std::io::Error::last_os_error()
                ));
            }

            let cq_head = *(self.cq_ring.add(self.params.cq_off.head as usize) as *const u32);
            let cq_mask = *(self.cq_ring.add(self.params.cq_off.ring_mask as usize) as *const u32);
            let cqe_idx = cq_head & cq_mask;
            let cqe = self
                .cq_ring
                .add(self.params.cq_off.cqes as usize + cqe_idx as usize * std::mem::size_of::<IoUringCqe>())
                as *const IoUringCqe;
            let res = (*cqe).res;
            *(self.cq_ring.add(self.params.cq_off.head as usize) as *mut u32) = cq_head.wrapping_add(1);

            if res < 0 {
                return Err(format!(
                    "accept failed: {}",
                    std::io::Error::from_raw_os_error(-res)
                ));
            }
            Ok(res)
        }
    }
}

#[cfg(target_os = "linux")]
impl Drop for IoUring {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.sq_ring as *mut _, self.sq_ring_sz);
            libc::munmap(self.cq_ring as *mut _, self.cq_ring_sz);
            libc::munmap(self.sqes as *mut _, self.sqes_sz);
            libc::close(self.fd);
        }
    }
}

fn block_on<F: std::future::Future>(f: F) -> F::Output {
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
