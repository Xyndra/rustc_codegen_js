#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlQuoteElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlQuoteElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `cite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
impl HtmlQuoteElement {
    #[js::import = "$0.cite"]
pub fn cite(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `cite` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLQuoteElement/cite)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlQuoteElement`*"]
impl HtmlQuoteElement {
    #[js::import = "$0.cite = $1"]
pub fn set_cite(&self, value: &str) { panic!("web-sys intrinsic") }
}
