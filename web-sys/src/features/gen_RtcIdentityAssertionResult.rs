#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityAssertionResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIdentityAssertionResult(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `assertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
#[js::import = "None"]
pub fn get_assertion(this: &RtcIdentityAssertionResult) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `assertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
#[js::import = "None = $1"]
pub fn set_assertion(this: &RtcIdentityAssertionResult, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Get the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
#[js::import = "None"]
pub fn get_idp(this: &RtcIdentityAssertionResult) -> RtcIdentityProviderDetails { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Change the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
#[js::import = "None = $1"]
pub fn set_idp(this: &RtcIdentityAssertionResult, val: &RtcIdentityProviderDetails) { panic!("web-sys intrinsic") }
impl RtcIdentityAssertionResult {
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Construct a new `RtcIdentityAssertionResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
#[js::import = "None"]
pub fn new(assertion: &str, idp: &RtcIdentityProviderDetails) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_assertion(assertion);
        ret.set_idp(idp);
        ret
    }
    #[deprecated = "Use `set_assertion()` instead."]
#[js::import = "None"]
pub fn assertion(&mut self, val: &str) -> &mut Self { self.set_assertion(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[deprecated = "Use `set_idp()` instead."]
#[js::import = "None"]
pub fn idp(&mut self, val: &RtcIdentityProviderDetails) -> &mut Self { self.set_idp(val) { panic!("web-sys intrinsic") }
        self
    }
}
