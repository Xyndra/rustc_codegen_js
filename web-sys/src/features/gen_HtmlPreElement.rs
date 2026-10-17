#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlPreElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlPreElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlPreElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlPreElement`*"]
impl HtmlPreElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLPreElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlPreElement`*"]
impl HtmlPreElement {
    #[js::import = "$0.width = $1"]
pub fn set_width(&self, value: i32) { panic!("web-sys intrinsic") }
}
