#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlBaseElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlBaseElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
impl HtmlBaseElement {
    #[js::import = "$0.href"]
pub fn href(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
impl HtmlBaseElement {
    #[js::import = "$0.href = $1"]
pub fn set_href(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
impl HtmlBaseElement {
    #[js::import = "$0.target"]
pub fn target(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLBaseElement/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlBaseElement`*"]
impl HtmlBaseElement {
    #[js::import = "$0.target = $1"]
pub fn set_target(&self, value: &str) { panic!("web-sys intrinsic") }
}
