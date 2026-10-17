#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlParagraphElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlParagraphElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
impl HtmlParagraphElement {
    #[js::import = "$0.align"]
pub fn align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLParagraphElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlParagraphElement`*"]
impl HtmlParagraphElement {
    #[js::import = "$0.align = $1"]
pub fn set_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
