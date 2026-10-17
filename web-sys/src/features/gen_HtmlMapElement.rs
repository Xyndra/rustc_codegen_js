#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlMapElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMapElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlMapElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMapElement`*"]
impl HtmlMapElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMapElement`*"]
impl HtmlMapElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `areas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLMapElement/areas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlMapElement`*"]
impl HtmlMapElement {
    #[js::import = "$0.areas"]
pub fn areas(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
