#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlBrElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBrElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlBrElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `clear` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBrElement`*"]
impl HtmlBrElement {
    #[js::import = "$0.clear"]
pub fn clear(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `clear` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBRElement/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBrElement`*"]
impl HtmlBrElement {
    #[js::import = "$0.clear = $1"]
pub fn set_clear(&self, value: &str) { panic!("web-sys intrinsic") }
}
