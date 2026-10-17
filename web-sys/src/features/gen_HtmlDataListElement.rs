#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDataListElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDataListElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlDataListElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `options` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDataListElement/options)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlDataListElement`*"]
impl HtmlDataListElement {
    #[js::import = "$0.options"]
pub fn options(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
