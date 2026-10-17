#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTitleElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTitleElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlTitleElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTitleElement`*"]
impl HtmlTitleElement {
    #[js::import = "$0.text"]
pub fn text(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTitleElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTitleElement`*"]
impl HtmlTitleElement {
    #[js::import = "$0.text = $1"]
pub fn set_text(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
