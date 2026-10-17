#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFormControlsCollection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlFormControlsCollection(pub ::wasm_bindgen::JsValue);
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormControlsCollection/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
impl HtmlFormControlsCollection {
    #[js::import = "$0.namedItem($1)"]
pub fn named_item(&self, name: &str) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
        method,
        structural,
        js_class = "HTMLFormControlsCollection",
        indexing_getter
    )]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormControlsCollection`*"]
#[js::import = "None"]
pub fn get(this: &HtmlFormControlsCollection, name: &str) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
