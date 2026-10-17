#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AssignedNodesOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AssignedNodesOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `flatten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
#[js::import = "None"]
pub fn get_flatten(this: &AssignedNodesOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `flatten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
#[js::import = "None = $1"]
pub fn set_flatten(this: &AssignedNodesOptions, val: bool) { panic!("web-sys intrinsic") }
impl AssignedNodesOptions {
    #[doc = "Construct a new `AssignedNodesOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_flatten()` instead."]
#[js::import = "None"]
pub fn flatten(&mut self, val: bool) -> &mut Self { self.set_flatten(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AssignedNodesOptions {
    fn default() -> Self {
        Self::new()
    }
}
