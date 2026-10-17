#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorAttestationResponse` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AuthenticatorAttestationResponse(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `attestationObject` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/attestationObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
impl AuthenticatorAttestationResponse {
    #[js::import = "$0.attestationObject"]
pub fn attestation_object(&self) -> ::js_sys::ArrayBuffer { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAuthenticatorData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getAuthenticatorData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
impl AuthenticatorAttestationResponse {
    #[js::import = "$0.getAuthenticatorData()"]
pub fn get_authenticator_data(&self) -> Result<::js_sys::ArrayBuffer, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getPublicKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getPublicKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
impl AuthenticatorAttestationResponse {
    #[js::import = "$0.getPublicKey()"]
pub fn get_public_key(&self) -> Result<Option<::js_sys::ArrayBuffer>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getPublicKeyAlgorithm()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getPublicKeyAlgorithm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
impl AuthenticatorAttestationResponse {
    #[js::import = "$0.getPublicKeyAlgorithm()"]
pub fn get_public_key_algorithm(&self) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getTransports()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorAttestationResponse/getTransports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttestationResponse`*"]
impl AuthenticatorAttestationResponse {
    #[js::import = "$0.getTransports()"]
pub fn get_transports(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
