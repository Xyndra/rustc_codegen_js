#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProviderDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIdentityProviderDetails(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
#[js::import = "None"]
pub fn get_domain(this: &RtcIdentityProviderDetails) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
#[js::import = "None = $1"]
pub fn set_domain(this: &RtcIdentityProviderDetails, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
#[js::import = "None"]
pub fn get_protocol(this: &RtcIdentityProviderDetails) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
#[js::import = "None = $1"]
pub fn set_protocol(this: &RtcIdentityProviderDetails, val: &str) { panic!("web-sys intrinsic") }
impl RtcIdentityProviderDetails {
    #[doc = "Construct a new `RtcIdentityProviderDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderDetails`*"]
#[js::import = "None"]
pub fn new(domain: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_domain(domain);
        ret
    }
    #[deprecated = "Use `set_domain()` instead."]
#[js::import = "None"]
pub fn domain(&mut self, val: &str) -> &mut Self { self.set_domain(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_protocol()` instead."]
#[js::import = "None"]
pub fn protocol(&mut self, val: &str) -> &mut Self { self.set_protocol(val) { panic!("web-sys intrinsic") }
        self
    }
}
