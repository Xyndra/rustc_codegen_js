#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HmacDerivedKeyParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HmacDerivedKeyParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
#[js::import = "None"]
pub fn get_name(this: &HmacDerivedKeyParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &HmacDerivedKeyParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
#[js::import = "None"]
pub fn get_hash(this: &HmacDerivedKeyParams) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
#[js::import = "None = $1"]
pub fn set_hash(this: &HmacDerivedKeyParams, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
#[js::import = "None"]
pub fn get_length(this: &HmacDerivedKeyParams) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
#[js::import = "None = $1"]
pub fn set_length(this: &HmacDerivedKeyParams, val: u32) { panic!("web-sys intrinsic") }
impl HmacDerivedKeyParams {
    #[doc = "Construct a new `HmacDerivedKeyParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacDerivedKeyParams`*"]
#[js::import = "None"]
pub fn new(name: &str, hash: &::wasm_bindgen::JsValue) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_hash(hash);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_hash()` instead."]
#[js::import = "None"]
pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_hash(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_length()` instead."]
#[js::import = "None"]
pub fn length(&mut self, val: u32) -> &mut Self { self.set_length(val) { panic!("web-sys intrinsic") }
        self
    }
}
