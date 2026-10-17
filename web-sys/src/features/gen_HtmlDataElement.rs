#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDataElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDataElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlDataElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDataElement`*"]
impl HtmlDataElement {
    #[js::import = "$0.value"]
pub fn value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDataElement`*"]
impl HtmlDataElement {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
