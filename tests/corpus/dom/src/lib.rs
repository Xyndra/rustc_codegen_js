// DOM corpus: opaque JS handles (`#[js::opaque]`), `$N` template import
// paths, and peered inline JS (`#[js::inline]` stubs called directly). The
// native oracle (`native.rs`) fakes the DOM; `run.js` fakes `document`
// for node.
#![feature(register_tool)]
#![register_tool(js)]
#![allow(dead_code, improper_ctypes)]

#[js::opaque]
#[derive(Clone, Copy, PartialEq)]
pub struct Element;

extern "C" {
    #[js::import = "document.getElementById($0)"]
    fn get_element(id: &str) -> Element;
    #[js::import = "$0.replaceChildren()"]
    fn el_clear(el: &Element);
    #[js::import = "$0.insertAdjacentHTML($1, $2)"]
    fn el_show(el: &Element, pos: &str, html: &str);
    #[js::import = "$0.getAttribute($1)"]
    fn el_attr(el: &Element, name: &str) -> String;
}

#[js::inline]
unsafe fn js0<T>(_: &str) -> T {
    panic!("js intrinsic")
}
#[js::inline]
unsafe fn js1<T, A0>(_: &str, _: A0) -> T {
    panic!("js intrinsic")
}

#[js::export]
pub fn run() -> String {
    let el = unsafe { get_element("stage") };
    unsafe { el_clear(&el) };
    unsafe { el_show(&el, "afterbegin", "<b>hi</b>") };
    // Clone is identity, equality is reference identity.
    let el2 = el.clone();
    let same = el == el2;
    // Handles compose inside ordinary ADTs.
    let oe: Option<Element> = Some(el);
    let present = oe.is_some();
    let mut v = Vec::new();
    v.push(el);
    // Inline reads (template + splice, no import declaration).
    let tag: String = unsafe { js1("document.getElementById($0).tagName", "stage") };
    let n: i32 = unsafe { js0("1 + 2") };
    let attr: String = unsafe { el_attr(&el, "id") };
    format!("{same} {present} {} {tag} {n} {attr}", v.len())
}
