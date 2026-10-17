#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GetRootNodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetRootNodeOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct GetRootNodeOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetRootNodeOptions`*"]
#[js::import = "None"]
pub fn get_composed(this: &GetRootNodeOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetRootNodeOptions`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &GetRootNodeOptions, val: bool) { panic!("web-sys intrinsic") }
impl GetRootNodeOptions {
    #[doc = "Construct a new `GetRootNodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetRootNodeOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for GetRootNodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
