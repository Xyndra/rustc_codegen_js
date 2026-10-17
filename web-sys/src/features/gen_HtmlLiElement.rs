#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLiElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlLiElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
impl HtmlLiElement {
    #[js::import = "$0.value"]
pub fn value(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
impl HtmlLiElement {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
impl HtmlLiElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLIElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLiElement`*"]
impl HtmlLiElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
