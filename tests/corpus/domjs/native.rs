// Native oracle for the domjs corpus: fakes the DOM the JS side reads
// through `run.js`, so both sides print the same summary line.
#[derive(Clone, Copy, PartialEq)]
struct Element;
#[derive(Clone, Copy, PartialEq)]
struct TextArea;

fn get_element(_id: &str) -> Option<Element> {
    Some(Element)
}
fn el_clear(_el: Element) {}
fn el_show(_el: Element, _pos: &str, _html: &str) {}
fn el_attr(_el: Element, _name: &str) -> Option<String> {
    Some("stage".to_string())
}
fn area_value(_area: TextArea) -> String {
    "push".to_string()
}
fn closest(_el: Element, sel: &str) -> Result<Option<Element>, String> {
    if sel == "!!!invalid" {
        Err("SyntaxError".to_string())
    } else if sel == ".nomatch" {
        Ok(None)
    } else {
        Ok(Some(Element))
    }
}
fn query_selector(_sel: &str) -> Result<Option<Element>, String> {
    Ok(Some(Element))
}

pub fn run() -> String {
    let el = get_element("stage").expect("missing stage");
    el_clear(el);
    el_show(el, "afterbegin", "<b>hi</b>");
    let el2 = el.clone();
    let same = el == el2;
    let oe: Option<Element> = Some(el);
    let present = oe.is_some();
    let mut v = Vec::new();
    v.push(el);
    let tag: String = "DIV".to_string();
    let n: i32 = 1 + 2;
    let attr: String = el_attr(el, "id").expect("missing id");
    let area = TextArea;
    let _ = get_element("cmd").expect("missing cmd");
    let found = closest(el, "div");
    let missing = closest(el, ".nomatch");
    let bad = closest(el, "!!!invalid");
    let q = query_selector("#stage");
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
        area_value(area)
    )
}
