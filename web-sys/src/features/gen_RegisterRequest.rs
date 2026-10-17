#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegisterRequest` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RegisterRequest(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
#[js::import = "None"]
pub fn get_challenge(this: &RegisterRequest) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
#[js::import = "None = $1"]
pub fn set_challenge(this: &RegisterRequest, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
#[js::import = "None"]
pub fn get_version(this: &RegisterRequest) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
#[js::import = "None = $1"]
pub fn set_version(this: &RegisterRequest, val: &str) { panic!("web-sys intrinsic") }
impl RegisterRequest {
    #[doc = "Construct a new `RegisterRequest`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterRequest`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_challenge()` instead."]
#[js::import = "None"]
pub fn challenge(&mut self, val: &str) -> &mut Self { self.set_challenge(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_version()` instead."]
#[js::import = "None"]
pub fn version(&mut self, val: &str) -> &mut Self { self.set_version(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RegisterRequest {
    fn default() -> Self {
        Self::new()
    }
}
