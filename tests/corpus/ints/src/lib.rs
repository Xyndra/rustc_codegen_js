#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

#[js::export]
pub fn run() -> String {
    let mut s = String::new();
    // 64-bit
    let a: i64 = -9_000_000_000;
    let b: i64 = 3_000_000_000;
    s.push_str(&format!("{} {} {} {} ", a + b, a - b, a * 2, b / 3));
    s.push_str(&format!("{} {} ", a % 7, -a % 7));
    let u: u64 = 18_446_744_073_709_551_615;
    s.push_str(&format!("{} {} ", u, u.wrapping_add(1)));
    s.push_str(&format!("{} ", i64::MIN.wrapping_sub(1)));
    s.push_str(&format!(
        "{:?} {:?} ",
        (-5i64).checked_div(0),
        7u64.checked_div(0)
    ));
    s.push_str(&format!("{} {} ", 1i64 << 40, -1i64 >> 3));
    s.push_str(&format!(
        "{} {} ",
        (0xFF00u64 & 0x0F0Fu64) | 0x00FFu64,
        0xFFu64 ^ 0x0Fu64
    ));
    s.push_str(&format!("{} ", 12345678901234567890u64 as i64 as i128));
    // 128-bit
    let big: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    s.push_str(&format!("{} {} ", big, big.wrapping_add(1)));
    s.push_str(&format!("{} ", i128::MIN.wrapping_sub(1)));
    s.push_str(&format!("{} ", u128::MAX));
    s.push_str(&format!(
        "{} {} ",
        340282366920938463463374607431768211455u128 as i128,
        5i128.pow(30)
    ));
    s.push_str(&format!("{} ", (1u128 << 100) + 7));
    s.push_str(&format!("{:?} ", (Some(-1i128)).map(|x| x - 1)));
    // cross-width casts
    let x: i32 = -12345;
    s.push_str(&format!(
        "{} {} {} ",
        x as i8 as i64, x as u16 as u64, x as i128
    ));
    s.push_str(&format!("{} ", (u32::MAX as u64) * 2));
    s.push_str(&format!("{} ", 1_000_000_000i64 as i32));
    s.push_str(&format!("{} ", (7i128 / 2 + 7i128 % 2) as i64));
    // f32
    s.push_str(&format!(
        "{} {} {} ",
        0.5f32 + 0.25f32,
        1.5f32 * 2.0f32,
        3.0f32 / 0.5f32
    ));
    s.push_str(&format!("{} ", -0.125f32));
    s.push_str(&format!("{:?} ", 0.5f32 / 3.0f32));
    s.push_str(&format!("{} ", 1.0f32 / 0.0f32));
    s.push_str(&format!("{} ", -1.0f32 / 0.0f32));
    s.push_str(&format!("{:?} ", (0.0f32 / 0.0f32).is_nan()));
    s.push_str(&format!("{} ", 2.5f32 as i32));
    s.push_str(&format!("{} ", (-2.5f32 as i32) as f64));
    s.push_str(&format!("{} ", 1.0f64 as f32));
    s.push_str(&format!("{} {} ", 10i32 as f32, 0.1f64 as f32));
    s.push_str(&format!("{} ", (3.75f32).sqrt()));
    s
}
