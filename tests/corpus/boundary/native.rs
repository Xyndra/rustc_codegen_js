// Native oracle for the boundary corpus. It stubs the `#[js::import]`
// declarations that `globalThis.__js_imports` provides at JS run time.
fn js_now() -> f64 {
    42.0
}

fn js_log(_s: &str) {}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn is_long(s: String) -> bool {
    s.len() > 5
}

pub fn run() -> String {
    let now = js_now();
    let g = greet("world");
    let b = is_long(g.clone());
    format!("{g} {} {} {} {}", add(2, 3), now, b, g)
}
