#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcdsaParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct EcdsaParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
#[js::import = "None"]
pub fn get_name(this: &EcdsaParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &EcdsaParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
#[js::import = "None"]
pub fn get_hash(this: &EcdsaParams) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
#[js::import = "None = $1"]
pub fn set_hash(this: &EcdsaParams, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl EcdsaParams {
    #[doc = "Construct a new `EcdsaParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdsaParams`*"]
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
}
