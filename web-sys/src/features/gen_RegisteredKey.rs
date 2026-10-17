#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegisteredKey` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RegisteredKey(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `appId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::import = "None"]
pub fn get_app_id(this: &RegisteredKey) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `appId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::import = "None = $1"]
pub fn set_app_id(this: &RegisteredKey, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::import = "None"]
pub fn get_key_handle(this: &RegisteredKey) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::import = "None = $1"]
pub fn set_key_handle(this: &RegisteredKey, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::import = "None"]
pub fn get_transports(this: &RegisteredKey) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::import = "None = $1"]
pub fn set_transports(this: &RegisteredKey, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::import = "None"]
pub fn get_version(this: &RegisteredKey) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::import = "None = $1"]
pub fn set_version(this: &RegisteredKey, val: &str) { panic!("web-sys intrinsic") }
impl RegisteredKey {
    #[doc = "Construct a new `RegisteredKey`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisteredKey`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_app_id()` instead."]
#[js::import = "None"]
pub fn app_id(&mut self, val: Option<&str>) -> &mut Self { self.set_app_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_key_handle()` instead."]
#[js::import = "None"]
pub fn key_handle(&mut self, val: &str) -> &mut Self { self.set_key_handle(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_transports()` instead."]
#[js::import = "None"]
pub fn transports(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self { self.set_transports(val.unwrap_or(&::wasm_bindgen::JsValue::NULL)) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_version()` instead."]
#[js::import = "None"]
pub fn version(&mut self, val: &str) -> &mut Self { self.set_version(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RegisteredKey {
    fn default() -> Self {
        Self::new()
    }
}
