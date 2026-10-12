#![feature(register_tool)]
#![register_tool(js)]

use dgs::{circle, color, line, point, scene};

#[js::export]
pub fn demo() -> String {
    let s = scene()
        .push(point("A", 0.0, 0.0).with_color(color("red")))
        .push(point("B", 3.0, 4.0))
        .push(line("A", "B"))
        .push(circle("A", 2.0));
    s.to_svg()
}

#[js::export]
pub fn run() -> String {
    demo()
}
