#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityValidationResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIdentityValidationResult(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `contents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
#[js::import = "None"]
pub fn get_contents(this: &RtcIdentityValidationResult) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `contents` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
#[js::import = "None = $1"]
pub fn set_contents(this: &RtcIdentityValidationResult, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `identity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
#[js::import = "None"]
pub fn get_identity(this: &RtcIdentityValidationResult) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `identity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
#[js::import = "None = $1"]
pub fn set_identity(this: &RtcIdentityValidationResult, val: &str) { panic!("web-sys intrinsic") }
impl RtcIdentityValidationResult {
    #[doc = "Construct a new `RtcIdentityValidationResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityValidationResult`*"]
#[js::import = "None"]
pub fn new(contents: &str, identity: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_contents(contents);
        ret.set_identity(identity);
        ret
    }
    #[deprecated = "Use `set_contents()` instead."]
#[js::import = "None"]
pub fn contents(&mut self, val: &str) -> &mut Self { self.set_contents(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_identity()` instead."]
#[js::import = "None"]
pub fn identity(&mut self, val: &str) -> &mut Self { self.set_identity(val) { panic!("web-sys intrinsic") }
        self
    }
}
