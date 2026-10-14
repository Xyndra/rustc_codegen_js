#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

#[js::export]
pub fn run() -> String {
    let mut s = String::new();
    // basic ops
    s.push_str(&format!("{} ", 2 + 3 * 4));
    s.push_str(&format!("{} ", (10 - 3) * 2));
    s.push_str(&format!("{} ", 17 / 5));
    s.push_str(&format!("{} ", 17 % 5));
    s.push_str(&format!("{} ", -17 / 5));
    s.push_str(&format!("{} ", -17 % 5));
    // wrapping (overflow-checks off semantics)
    s.push_str(&format!("{} ", i32::MAX.wrapping_add(1)));
    s.push_str(&format!("{} ", i32::MIN.wrapping_sub(1)));
    s.push_str(&format!("{} ", 255u8.wrapping_add(1)));
    // shifts & bits
    s.push_str(&format!("{} ", (1i32 << 30) + (1i32 << 2)));
    s.push_str(&format!("{} ", (-8i32) >> 2));
    s.push_str(&format!("{} ", (0xF0u8 & 0x3Cu8) | 0x11u8));
    s.push_str(&format!("{} ", (0xF0u8 ^ 0xFFu8)));
    // comparisons
    s.push_str(&format!("{} ", (3 < 4) as u8));
    s.push_str(&format!("{} ", (3 == 4) as u8));
    // checked / saturating
    s.push_str(&format!("{:?} ", 200u8.checked_add(100)));
    s.push_str(&format!("{} ", 200u8.saturating_add(100)));
    s.push_str(&format!("{} ", 5u8.saturating_sub(10)));
    // bool logic
    s.push_str(&format!("{} ", (true && false) as u8));
    s.push_str(&format!("{} ", (true || false) as u8));
    // float methods (intrinsics lowered per-width)
    s.push_str(&format!("{} ", 3.7f64.floor()));
    s.push_str(&format!("{} ", (-3.2f64).ceil()));
    s.push_str(&format!("{} ", 2.5f64.round()));
    s.push_str(&format!("{} ", (-2.5f64).round()));
    s.push_str(&format!("{} ", 2.0f64.sqrt()));
    s.push_str(&format!("{} ", 2.0f64.powf(10.0)));
    s.push_str(&format!("{} ", 3.0f64.min(4.0)));
    s.push_str(&format!("{} ", f64::NAN.min(4.0)));
    s.push_str(&format!("{} ", 3.0f64.max(f64::NAN)));
    s.push_str(&format!("{} ", 5.0f64.clamp(0.0, 4.0)));
    s.push_str(&format!("{} ", (1.0f64).atan2(1.0)));
    s.push_str(&format!("{} ", 7usize.max(9)));
    s.push_str(&format!("{} ", (-3i32).min(4)));
    s.push_str(&format!("{} ", 2.5f32.floor()));
    s
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[allow(dead_code)]
fn fib_check() -> u32 {
    fib(20)
}
