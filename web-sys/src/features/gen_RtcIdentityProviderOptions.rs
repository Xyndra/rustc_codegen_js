#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProviderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIdentityProviderOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
#[js::import = "None"]
pub fn get_peer_identity(this: &RtcIdentityProviderOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
#[js::import = "None = $1"]
pub fn set_peer_identity(this: &RtcIdentityProviderOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
#[js::import = "None"]
pub fn get_protocol(this: &RtcIdentityProviderOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
#[js::import = "None = $1"]
pub fn set_protocol(this: &RtcIdentityProviderOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `usernameHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
#[js::import = "None"]
pub fn get_username_hint(this: &RtcIdentityProviderOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `usernameHint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
#[js::import = "None = $1"]
pub fn set_username_hint(this: &RtcIdentityProviderOptions, val: &str) { panic!("web-sys intrinsic") }
impl RtcIdentityProviderOptions {
    #[doc = "Construct a new `RtcIdentityProviderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_peer_identity()` instead."]
#[js::import = "None"]
pub fn peer_identity(&mut self, val: &str) -> &mut Self { self.set_peer_identity(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_protocol()` instead."]
#[js::import = "None"]
pub fn protocol(&mut self, val: &str) -> &mut Self { self.set_protocol(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_username_hint()` instead."]
#[js::import = "None"]
pub fn username_hint(&mut self, val: &str) -> &mut Self { self.set_username_hint(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcIdentityProviderOptions {
    fn default() -> Self {
        Self::new()
    }
}
