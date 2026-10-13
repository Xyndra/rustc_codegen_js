#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code, improper_ctypes)]

mod eval;

// Document access for the playground page. Every import binds a TRUE JS
// function via its dotted path — no host shims exist anywhere. All wiring
// decisions live here: `init()` arms one button per scene with that scene's
// painter as its click callback, then paints the first scene. The page calls
// `init()` on load and wires nothing itself.
extern "C" {
    #[js::import = "document.getElementById(\"stage\").replaceChildren"]
    fn stage_clear();
    #[js::import = "document.getElementById(\"stage\").insertAdjacentHTML"]
    fn stage_show(pos: &str, html: &str);
    #[js::import = "document.getElementById(\"status\").replaceChildren"]
    fn status_show(text: &str);
    #[js::import = "document.getElementById(\"s0\").addEventListener"]
    fn arm_0(event: &str, cb: fn());
    #[js::import = "document.getElementById(\"s1\").addEventListener"]
    fn arm_1(event: &str, cb: fn());
    #[js::import = "document.getElementById(\"s2\").addEventListener"]
    fn arm_2(event: &str, cb: fn());
    #[js::import = "document.getElementById(\"s3\").addEventListener"]
    fn arm_3(event: &str, cb: fn());
    #[js::import = "document.getElementById(\"s4\").addEventListener"]
    fn arm_4(event: &str, cb: fn());
    #[js::import = "document.getElementById(\"s5\").addEventListener"]
    fn arm_5(event: &str, cb: fn());
}

#[js::export]
pub fn render(code: &str) -> String {
    match eval::render(code) {
        Ok(svg) => svg,
        Err(e) => format!("Error: {}", e),
    }
}

/// Page entry point for load: arm every scene button, paint scene 0.
#[js::export]
pub fn init() {
    unsafe { arm_0("click", show_0) };
    unsafe { arm_1("click", show_1) };
    unsafe { arm_2("click", show_2) };
    unsafe { arm_3("click", show_3) };
    unsafe { arm_4("click", show_4) };
    unsafe { arm_5("click", show_5) };
    paint(0);
}

/// Page entry point for consoles: repaint scene 0.
#[js::export]
pub fn run() {
    paint(0);
}

fn show_0() {
    paint(0);
}
fn show_1() {
    paint(1);
}
fn show_2() {
    paint(2);
}
fn show_3() {
    paint(3);
}
fn show_4() {
    paint(4);
}
fn show_5() {
    paint(5);
}

fn scene(i: usize) -> &'static str {
    match i {
        0 => "push(point(\"A\", 0, 0))",
        1 => "push(point(\"A\", 0, 0))\npush(circle(\"A\", 2))",
        2 => "push(line((0, 0), (3, 4)))",
        3 => "theme(\"dark\")\npush(circle((1, 1), 1.5))",
        4 => "grid(true)\naxes(true)\npush(point(\"B\", -2, 3))",
        _ => "push(polygon((0, 0), (4, 0), (2, 3)))",
    }
}

fn paint(i: usize) {
    let code = scene(i);
    match eval::render(code) {
        Ok(svg) => {
            unsafe { stage_clear() };
            unsafe { stage_show("afterbegin", &svg) };
            unsafe { status_show(&format!("scene {}/6", i + 1)) };
        }
        Err(e) => {
            unsafe { status_show(&format!("scene {}/6: Error: {e}", i + 1)) };
        }
    }
}
