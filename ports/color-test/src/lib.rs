#![feature(register_tool)]
#![register_tool(js)]

use dgs::{arc, color, line, point, polygon, scene};

#[js::export]
pub fn run() -> String {
    let s = scene()
        .push(point("A", 0.0, 0.0).with_color(color("red")))
        .push(point("B", 3.0, 4.0).with_color(color("blue")))
        .push(point("C", 6.0, 0.0))
        .push(line("A", "B").with_color(color("green")))
        .push(arc("A", 1.5, 0.0, 180.0).with_color(color("purple")))
        .push(polygon(vec!["A".into(), "B".into(), "C".into()]).with_color(color("orange")));
    s.to_svg()
}
