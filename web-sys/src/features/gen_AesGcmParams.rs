#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesGcmParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AesGcmParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::import = "None"]
pub fn get_name(this: &AesGcmParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &AesGcmParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::import = "None"]
pub fn get_additional_data(this: &AesGcmParams) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::import = "None = $1"]
pub fn set_additional_data(this: &AesGcmParams, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[doc = "Get the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::import = "None"]
pub fn get_iv(this: &AesGcmParams) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::import = "None = $1"]
pub fn set_iv(this: &AesGcmParams, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[doc = "Get the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::import = "None"]
pub fn get_tag_length(this: &AesGcmParams) -> Option<u8> { panic!("web-sys intrinsic") }
    #[doc = "Change the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::import = "None = $1"]
pub fn set_tag_length(this: &AesGcmParams, val: u8) { panic!("web-sys intrinsic") }
impl AesGcmParams {
    #[doc = "Construct a new `AesGcmParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
#[js::import = "None"]
pub fn new(name: &str, iv: &::js_sys::Object) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_iv(iv);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_additional_data()` instead."]
#[js::import = "None"]
pub fn additional_data(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_additional_data(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_iv()` instead."]
#[js::import = "None"]
pub fn iv(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_iv(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_tag_length()` instead."]
#[js::import = "None"]
pub fn tag_length(&mut self, val: u8) -> &mut Self { self.set_tag_length(val) { panic!("web-sys intrinsic") }
        self
    }
}
