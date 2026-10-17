#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlUListElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlUListElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
impl HtmlUListElement {
    #[js::import = "$0.compact"]
pub fn compact(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `compact` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/compact)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
impl HtmlUListElement {
    #[js::import = "$0.compact = $1"]
pub fn set_compact(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
impl HtmlUListElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLUListElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlUListElement`*"]
impl HtmlUListElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
