#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasicCardRequest` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BasicCardRequest(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `supportedNetworks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
#[js::import = "None"]
pub fn get_supported_networks(this: &BasicCardRequest) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `supportedNetworks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
#[js::import = "None = $1"]
pub fn set_supported_networks(this: &BasicCardRequest, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `supportedTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
#[js::import = "None"]
pub fn get_supported_types(this: &BasicCardRequest) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `supportedTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
#[js::import = "None = $1"]
pub fn set_supported_types(this: &BasicCardRequest, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl BasicCardRequest {
    #[doc = "Construct a new `BasicCardRequest`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_supported_networks()` instead."]
#[js::import = "None"]
pub fn supported_networks(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_supported_networks(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_supported_types()` instead."]
#[js::import = "None"]
pub fn supported_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_supported_types(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BasicCardRequest {
    fn default() -> Self {
        Self::new()
    }
}
