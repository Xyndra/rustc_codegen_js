#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityAssertion` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIdentityAssertion(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
#[js::import = "None"]
pub fn get_idp(this: &RtcIdentityAssertion) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
#[js::import = "None = $1"]
pub fn set_idp(this: &RtcIdentityAssertion, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
#[js::import = "None"]
pub fn get_name(this: &RtcIdentityAssertion) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &RtcIdentityAssertion, val: &str) { panic!("web-sys intrinsic") }
impl RtcIdentityAssertion {
    #[doc = "Construct a new `RtcIdentityAssertion`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertion`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_idp()` instead."]
#[js::import = "None"]
pub fn idp(&mut self, val: &str) -> &mut Self { self.set_idp(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcIdentityAssertion {
    fn default() -> Self {
        Self::new()
    }
}
