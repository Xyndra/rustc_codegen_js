#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDirectoryElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDirectoryElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlDirectoryElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDirectoryElement`*"]
impl HtmlDirectoryElement {
    #[js::import = "$0.compact"]
pub fn compact(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDirectoryElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDirectoryElement`*"]
impl HtmlDirectoryElement {
    #[js::import = "$0.compact = $1"]
pub fn set_compact(&self, value: bool) { panic!("web-sys intrinsic") }
}
