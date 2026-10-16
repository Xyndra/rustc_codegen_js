// Native oracle for the dom corpus: fakes the DOM the JS side reads through
// `run.js`, so both sides print the same summary line.
#[derive(Clone, Copy, PartialEq)]
struct Element;

fn get_element(_id: &str) -> Element {
    Element
}
fn el_clear(_el: &Element) {}
fn el_show(_el: &Element, _pos: &str, _html: &str) {}
fn el_attr(_el: &Element, _name: &str) -> String {
    "stage".to_string()
}

pub fn run() -> String {
    let el = get_element("stage");
    el_clear(&el);
    el_show(&el, "afterbegin", "<b>hi</b>");
    let el2 = el.clone();
    let same = el == el2;
    let oe: Option<Element> = Some(el);
    let present = oe.is_some();
    let mut v = Vec::new();
    v.push(el);
    let tag: String = "DIV".to_string();
    let n: i32 = 1 + 2;
    let attr = el_attr(&el, "id");
    format!("{same} {present} {} {tag} {n} {attr}", v.len())
}
