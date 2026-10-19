#![no_std]
extern crate alloc;

use alloc::string::String;

pub fn escape_html(input: &str) -> String {
    let mut out = String::new();
    for c in input.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(feature = "web")]
pub mod web {
    use js_util::JsValue;

    pub fn set_inner_html(el: JsValue, html: &str) {
        let _: () = js_util::js2("$0.innerHTML = $1", el, html);
    }
}
