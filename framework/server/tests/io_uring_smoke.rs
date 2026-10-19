#[cfg(target_os = "linux")]
#[test]
fn io_uring_server_accepts_and_responds() {
    use fw_server::{IoUringServer, Server};
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let mut srv = IoUringServer::new();
    srv.router().add(fw_router::Route {
        path: vec![],
        method: fw_router::Method::Get,
        middleware: Vec::new(),
        handler: 0,
        cache: fw_router::CachePolicy::None,
    });
    srv.bind("127.0.0.1:18931").unwrap();
    std::thread::spawn(move || {
        let _ = srv.serve();
    });

    let mut connected = None;
    for _ in 0..50 {
        match TcpStream::connect("127.0.0.1:18931") {
            Ok(c) => {
                connected = Some(c);
                break;
            }
            Err(_) => std::thread::sleep(std::time::Duration::from_millis(100)),
        }
    }
    let mut c = connected.expect("server did not come up");
    c.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
    c.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();
    let mut buf = Vec::new();
    let _ = c.read_to_end(&mut buf);
    let text = String::from_utf8_lossy(&buf);
    assert!(text.contains("HTTP/1.1 200"), "response was: {text:?}");
}
