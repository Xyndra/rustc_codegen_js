#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProvider` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIdentityProvider(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `generateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
#[js::import = "None"]
pub fn get_generate_assertion(this: &RtcIdentityProvider) -> ::js_sys::Function { panic!("web-sys intrinsic") }
    #[doc = "Change the `generateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
#[js::import = "None = $1"]
pub fn set_generate_assertion(this: &RtcIdentityProvider, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `validateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
#[js::import = "None"]
pub fn get_validate_assertion(this: &RtcIdentityProvider) -> ::js_sys::Function { panic!("web-sys intrinsic") }
    #[doc = "Change the `validateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
#[js::import = "None = $1"]
pub fn set_validate_assertion(this: &RtcIdentityProvider, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
impl RtcIdentityProvider {
    #[doc = "Construct a new `RtcIdentityProvider`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
#[js::import = "None"]
pub fn new(generate_assertion: &::js_sys::Function, validate_assertion: &::js_sys::Function,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_generate_assertion(generate_assertion);
        ret.set_validate_assertion(validate_assertion);
        ret
    }
    #[deprecated = "Use `set_generate_assertion()` instead."]
#[js::import = "None"]
pub fn generate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_generate_assertion(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_validate_assertion()` instead."]
#[js::import = "None"]
pub fn validate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_validate_assertion(val) { panic!("web-sys intrinsic") }
        self
    }
}
