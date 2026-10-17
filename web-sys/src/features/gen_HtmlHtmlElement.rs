#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlHtmlElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHtmlElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlHtmlElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHtmlElement`*"]
impl HtmlHtmlElement {
    #[js::import = "$0.version"]
pub fn version(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLHtmlElement/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlHtmlElement`*"]
impl HtmlHtmlElement {
    #[js::import = "$0.version = $1"]
pub fn set_version(&self, value: &str) { panic!("web-sys intrinsic") }
}
