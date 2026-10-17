#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainDomStringParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConstrainDomStringParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
#[js::import = "None"]
pub fn get_exact(this: &ConstrainDomStringParameters) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
#[js::import = "None = $1"]
pub fn set_exact(this: &ConstrainDomStringParameters, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
#[js::import = "None"]
pub fn get_ideal(this: &ConstrainDomStringParameters) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
#[js::import = "None = $1"]
pub fn set_ideal(this: &ConstrainDomStringParameters, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl ConstrainDomStringParameters {
    #[doc = "Construct a new `ConstrainDomStringParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_exact()` instead."]
#[js::import = "None"]
pub fn exact(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_exact(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ideal()` instead."]
#[js::import = "None"]
pub fn ideal(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_ideal(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConstrainDomStringParameters {
    fn default() -> Self {
        Self::new()
    }
}
