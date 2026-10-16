#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

mod eval;

use dom::{HtmlTextAreaElement, document};

// Document access for the playground page goes entirely through the `dom`
// crate (typed handles, web-sys naming) — no `extern` blocks, no imports,
// no unsafe. `init()` arms one button per scene with that scene's painter
// as its click callback, then paints the first scene. The page calls
// `init()` on load and wires nothing itself.
fn stage() -> dom::Element {
    document()
        .get_element_by_id("stage")
        .expect("missing #stage")
}

fn status() -> dom::Element {
    document()
        .get_element_by_id("status")
        .expect("missing #status")
}

fn button(id: &str) -> dom::Element {
    document().get_element_by_id(id).expect("missing button")
}

fn cmd_text() -> String {
    HtmlTextAreaElement::from_element(button("cmd")).value()
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
    button("s0").add_event_listener("click", show_0);
    button("s1").add_event_listener("click", show_1);
    button("s2").add_event_listener("click", show_2);
    button("s3").add_event_listener("click", show_3);
    button("s4").add_event_listener("click", show_4);
    button("s5").add_event_listener("click", show_5);
    button("render").add_event_listener("click", run_cmd);
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

#[js::export]
pub fn run_from_source() {
    let code = cmd_text();
    match eval::render(&code) {
        Ok(svg) => {
            let stage = stage();
            stage.replace_children();
            stage.insert_adjacent_html("afterbegin", &svg);
            status().set_text_content("rendered");
        }
        Err(e) => {
            status().set_text_content(&format!("Error: {e}"));
        }
    }
}

fn run_cmd() {
    run_from_source();
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
            let stage = stage();
            stage.replace_children();
            stage.insert_adjacent_html("afterbegin", &svg);
            status().set_text_content(&format!("scene {}/6", i + 1));
        }
        Err(e) => {
            status().set_text_content(&format!("scene {}/6: Error: {e}", i + 1));
        }
    }
}
