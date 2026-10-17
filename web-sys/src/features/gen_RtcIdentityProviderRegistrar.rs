#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProviderRegistrar` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderRegistrar`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIdentityProviderRegistrar(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcIdentityProvider")]
    #[doc = "The `register()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIdentityProviderRegistrar/register)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`, `RtcIdentityProviderRegistrar`*"]
impl RtcIdentityProviderRegistrar {
    #[js::import = "$0.register($1)"]
pub fn register(&self, idp: &RtcIdentityProvider) { panic!("web-sys intrinsic") }
}
