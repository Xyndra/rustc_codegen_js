#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesKeyGenParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AesKeyGenParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
#[js::import = "None"]
pub fn get_name(this: &AesKeyGenParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &AesKeyGenParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
#[js::import = "None"]
pub fn get_length(this: &AesKeyGenParams) -> u16 { panic!("web-sys intrinsic") }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
#[js::import = "None = $1"]
pub fn set_length(this: &AesKeyGenParams, val: u16) { panic!("web-sys intrinsic") }
impl AesKeyGenParams {
    #[doc = "Construct a new `AesKeyGenParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesKeyGenParams`*"]
#[js::import = "None"]
pub fn new(name: &str, length: u16) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_length(length);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_length()` instead."]
#[js::import = "None"]
pub fn length(&mut self, val: u16) -> &mut Self { self.set_length(val) { panic!("web-sys intrinsic") }
        self
    }
}
