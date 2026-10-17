#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesCtrParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AesCtrParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
#[js::import = "None"]
pub fn get_name(this: &AesCtrParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &AesCtrParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
#[js::import = "None"]
pub fn get_counter(this: &AesCtrParams) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `counter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
#[js::import = "None = $1"]
pub fn set_counter(this: &AesCtrParams, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
#[js::import = "None"]
pub fn get_length(this: &AesCtrParams) -> u8 { panic!("web-sys intrinsic") }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
#[js::import = "None = $1"]
pub fn set_length(this: &AesCtrParams, val: u8) { panic!("web-sys intrinsic") }
impl AesCtrParams {
    #[doc = "Construct a new `AesCtrParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesCtrParams`*"]
#[js::import = "None"]
pub fn new(name: &str, counter: &::js_sys::Object, length: u8) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_counter(counter);
        ret.set_length(length);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_counter()` instead."]
#[js::import = "None"]
pub fn counter(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_counter(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_length()` instead."]
#[js::import = "None"]
pub fn length(&mut self, val: u8) -> &mut Self { self.set_length(val) { panic!("web-sys intrinsic") }
        self
    }
}
