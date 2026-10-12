#![feature(register_tool)]
#![register_tool(js)]

/// Iterate an FFI-arrived `&str` with `str::Chars` and return its code points
/// as a comma-separated list. This is the exact scenario that exercised the
/// broken `StrChars` + `CharsNext` shims before the fix.
#[js::export]
pub fn test_chars(code: &str) -> String {
    let mut out = String::new();
    let mut first = true;
    for ch in code.chars() {
        if !first {
            out.push(',');
        }
        first = false;
        out.push_str(&format!("{}", ch as u32));
    }
    out
}

/// The canonical dgs-playground input: a string arriving from JS (a cell
/// `[String(s)]`) that must be fully iterated by `str::Chars`.
///
/// Expected code points (decimal):
///   p u s h ( = 112,117,115,104,40
///   p o i n t ( = 112,111,105,110,116,40
///   " = 34
///   A = 65
///   " = 34
///   , = 44
///   <space> = 32
///   0 = 48
///   <space> = 32
///   0 = 48
///   ) ) = 41,41
#[js::export]
pub fn run() -> String {
    let s = "push(point(\"A\", 0, 0))";
    test_chars(s)
}
