#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTimeElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTimeElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlTimeElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `dateTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTimeElement`*"]
impl HtmlTimeElement {
    #[js::import = "$0.dateTime"]
pub fn date_time(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `dateTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTimeElement/dateTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTimeElement`*"]
impl HtmlTimeElement {
    #[js::import = "$0.dateTime = $1"]
pub fn set_date_time(&self, value: &str) { panic!("web-sys intrinsic") }
}
