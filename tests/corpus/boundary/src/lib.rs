// #[js::export] boundary functions (Rust → JS) and #[js::import] foreign
// declarations (JS → Rust). The backend validates every boundary type against
// the allowed JS-crossing set (f64, i32, u32, bool, String, &str).
#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code, improper_ctypes)]

// A foreign function declared from JavaScript and called as if native.
#[js::import]
extern "C" {
    fn js_now() -> f64;
    fn js_log(s: &str);
}

#[js::export]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[js::export]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[js::export]
pub fn is_long(s: String) -> bool {
    s.len() > 5
}

#[js::export]
pub fn run() -> String {
    let now = unsafe { js_now() };
    unsafe { js_log("hello"); }
    let g = greet("world");
    let b = is_long(g.clone());
    format!("{g} {} {} {} {}", add(2, 3), now, b, g)
}
