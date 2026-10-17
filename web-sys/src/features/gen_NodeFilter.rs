#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NodeFilter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NodeFilter(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `acceptNode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`*"]
#[js::import = "None"]
pub fn get_accept_node(this: &NodeFilter) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `acceptNode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`*"]
#[js::import = "None = $1"]
pub fn set_accept_node(this: &NodeFilter, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
impl NodeFilter {
    #[doc = "Construct a new `NodeFilter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_accept_node()` instead."]
#[js::import = "None"]
pub fn accept_node(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_accept_node(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for NodeFilter {
    fn default() -> Self {
        Self::new()
    }
}
