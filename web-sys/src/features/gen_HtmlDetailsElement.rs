#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDetailsElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDetailsElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlDetailsElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `open` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDetailsElement`*"]
impl HtmlDetailsElement {
    #[js::import = "$0.open"]
pub fn open(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `open` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDetailsElement/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDetailsElement`*"]
impl HtmlDetailsElement {
    #[js::import = "$0.open = $1"]
pub fn set_open(&self, value: bool) { panic!("web-sys intrinsic") }
}
