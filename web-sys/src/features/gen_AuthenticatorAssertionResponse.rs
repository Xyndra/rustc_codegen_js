#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorAssertionResponse` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AuthenticatorAssertionResponse(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `authenticatorData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/authenticatorData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
impl AuthenticatorAssertionResponse {
    #[js::import = "$0.authenticatorData"]
pub fn authenticator_data(&self) -> ::js_sys::ArrayBuffer { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `signature` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/signature)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
impl AuthenticatorAssertionResponse {
    #[js::import = "$0.signature"]
pub fn signature(&self) -> ::js_sys::ArrayBuffer { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `userHandle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/userHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
impl AuthenticatorAssertionResponse {
    #[js::import = "$0.userHandle"]
pub fn user_handle(&self) -> Option<::js_sys::ArrayBuffer> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `attestationObject` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAssertionResponse/attestationObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAssertionResponse`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AuthenticatorAssertionResponse {
    #[js::import = "$0.attestationObject"]
pub fn attestation_object(&self) -> Option<::js_sys::ArrayBuffer> { panic!("web-sys intrinsic") }
}
