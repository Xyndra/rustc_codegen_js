#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDivElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDivElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlDivElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDivElement`*"]
impl HtmlDivElement {
    #[js::import = "$0.align"]
pub fn align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDivElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDivElement`*"]
impl HtmlDivElement {
    #[js::import = "$0.align = $1"]
pub fn set_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
