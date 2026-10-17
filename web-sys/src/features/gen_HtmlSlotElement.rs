#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSlotElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlSlotElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
impl HtmlSlotElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
impl HtmlSlotElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `assignedNodes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSlotElement`*"]
impl HtmlSlotElement {
    #[js::import = "$0.assignedNodes()"]
pub fn assigned_nodes(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AssignedNodesOptions")]
    #[doc = "The `assignedNodes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSlotElement/assignedNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`, `HtmlSlotElement`*"]
impl HtmlSlotElement {
    #[js::import = "$0.assignedNodes($1)"]
pub fn assigned_nodes_with_options(&self, options: &AssignedNodesOptions) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
