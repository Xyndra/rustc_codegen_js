#![feature(register_tool)]
#![register_tool(js)]

mod eval;

#[js::export]
pub fn render(code: &str) -> String {
    match eval::render(code) {
        Ok(svg) => svg,
        Err(e) => format!("Error: {}", e),
    }
}
