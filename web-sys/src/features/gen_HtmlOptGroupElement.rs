#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOptGroupElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlOptGroupElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
impl HtmlOptGroupElement {
    #[js::import = "$0.disabled"]
pub fn disabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
impl HtmlOptGroupElement {
    #[js::import = "$0.disabled = $1"]
pub fn set_disabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
impl HtmlOptGroupElement {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptGroupElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`*"]
impl HtmlOptGroupElement {
    #[js::import = "$0.label = $1"]
pub fn set_label(&self, value: &str) { panic!("web-sys intrinsic") }
}
