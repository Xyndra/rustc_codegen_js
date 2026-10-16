// Support-crate corpus: the same DOM exercise as `dom`, but through the
// upstream `dom` crate (transparent newtypes, `#[js::import]` methods,
// `Option` returns) plus one explicit upstream `#[js::inline]` stub call.
// Exercises cross-crate import peering (attribute metadata). Differentially
// tested against the faked DOM in `native.rs` + `run.js`.
#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code)]

use dom::{HtmlTextAreaElement, document};

#[js::export]
pub fn run() -> String {
    let doc = document();
    let el = doc.get_element_by_id("stage").expect("missing stage");
    el.replace_children();
    el.insert_adjacent_html("afterbegin", "<b>hi</b>");
    // Clone is identity, equality is reference identity.
    let el2 = el.clone();
    let same = el == el2;
    // Handles compose inside ordinary ADTs.
    let oe: Option<dom::Element> = Some(el);
    let present = oe.is_some();
    let mut v = Vec::new();
    v.push(el);
    // Upstream inline stub, called explicitly (no macro involved).
    let tag: String = js_util::js1("document.getElementById($0).tagName", "stage");
    let n: i32 = js_util::js0("1 + 2");
    let attr: String = el
        .get_attribute("id")
        .expect("missing id")
        .to_string();
    let area = HtmlTextAreaElement::from_element(
        doc.get_element_by_id("cmd").expect("missing cmd"),
    );
    // Throwing DOM reads: `Err` on invalid selectors, `Ok(None)` on no
    // match. (Payloads never print: native and JS errors render
    // differently — only the outcome shapes are compared.)
    let found = el.closest("div");
    let missing = el.closest(".nomatch");
    let bad = el.closest("!!!invalid");
    let q = doc.query_selector("#stage");
    let catch = format!(
        "{}-{}-{}-{}",
        matches!(found, Ok(Some(_))),
        matches!(missing, Ok(None)),
        bad.is_err(),
        matches!(q, Ok(Some(_))),
    );
    format!(
        "{same} {present} {} {tag} {n} {attr} {}\n{catch}",
        v.len(),
        area.value()
    )
}
