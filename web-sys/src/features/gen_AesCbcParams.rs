#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesCbcParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AesCbcParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
#[js::import = "None"]
pub fn get_name(this: &AesCbcParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &AesCbcParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
#[js::import = "None"]
pub fn get_iv(this: &AesCbcParams) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
#[js::import = "None = $1"]
pub fn set_iv(this: &AesCbcParams, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
impl AesCbcParams {
    #[doc = "Construct a new `AesCbcParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCbcParams`*"]
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
    #[deprecated = "Use `set_iv()` instead."]
#[js::import = "None"]
pub fn iv(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_iv(val) { panic!("web-sys intrinsic") }
        self
    }
}
