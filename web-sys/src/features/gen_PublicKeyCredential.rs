#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredential` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PublicKeyCredential(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `rawId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/rawId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
impl PublicKeyCredential {
    #[js::import = "$0.rawId"]
pub fn raw_id(&self) -> ::js_sys::ArrayBuffer { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AuthenticatorResponse")]
    #[doc = "Getter for the `response` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorResponse`, `PublicKeyCredential`*"]
impl PublicKeyCredential {
    #[js::import = "$0.response"]
pub fn response(&self) -> AuthenticatorResponse { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/authenticatorAttachment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl PublicKeyCredential {
    #[js::import = "$0.authenticatorAttachment"]
pub fn authenticator_attachment(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AuthenticationExtensionsClientOutputs")]
    #[doc = "The `getClientExtensionResults()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/getClientExtensionResults)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`, `PublicKeyCredential`*"]
impl PublicKeyCredential {
    #[js::import = "$0.getClientExtensionResults()"]
pub fn get_client_extension_results(&self) -> AuthenticationExtensionsClientOutputs { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `isConditionalMediationAvailable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isConditionalMediationAvailable_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl PublicKeyCredential {
    #[js::import = "PublicKeyCredential.isConditionalMediationAvailable"]
pub fn is_conditional_mediation_available() -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `isPasskeyPlatformAuthenticatorAvailable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isPasskeyPlatformAuthenticatorAvailable_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl PublicKeyCredential {
    #[js::import = "PublicKeyCredential.isPasskeyPlatformAuthenticatorAvailable"]
pub fn is_passkey_platform_authenticator_available() -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `isUserVerifyingPlatformAuthenticatorAvailable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/isUserVerifyingPlatformAuthenticatorAvailable_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
impl PublicKeyCredential {
    #[js::import = "PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable"]
pub fn is_user_verifying_platform_authenticator_available() -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "PublicKeyCredentialCreationOptions",
        feature = "PublicKeyCredentialCreationOptionsJson",
    ))]
    #[doc = "The `parseCreationOptionsFromJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/parseCreationOptionsFromJSON_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`, `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialCreationOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl PublicKeyCredential {
    #[js::import = "PublicKeyCredential.parseCreationOptionsFromJSON"]
pub fn parse_creation_options_from_json(options: &PublicKeyCredentialCreationOptionsJson,) -> Result<PublicKeyCredentialCreationOptions, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "PublicKeyCredentialRequestOptions",
        feature = "PublicKeyCredentialRequestOptionsJson",
    ))]
    #[doc = "The `parseRequestOptionsFromJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/parseRequestOptionsFromJSON_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`, `PublicKeyCredentialRequestOptions`, `PublicKeyCredentialRequestOptionsJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl PublicKeyCredential {
    #[js::import = "PublicKeyCredential.parseRequestOptionsFromJSON"]
pub fn parse_request_options_from_json(options: &PublicKeyCredentialRequestOptionsJson,) -> Result<PublicKeyCredentialRequestOptions, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PublicKeyCredential/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredential`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl PublicKeyCredential {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
