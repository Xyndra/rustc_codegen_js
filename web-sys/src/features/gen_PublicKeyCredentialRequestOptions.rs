#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PublicKeyCredentialRequestOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `allowCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None"]
pub fn get_allow_credentials(this: &PublicKeyCredentialRequestOptions,) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `allowCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None = $1"]
pub fn set_allow_credentials(this: &PublicKeyCredentialRequestOptions, val: &::wasm_bindgen::JsValue,) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_attestation(this: &PublicKeyCredentialRequestOptions,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_attestation(this: &PublicKeyCredentialRequestOptions, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_attestation_formats(this: &PublicKeyCredentialRequestOptions,) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_attestation_formats(this: &PublicKeyCredentialRequestOptions, val: &::wasm_bindgen::JsValue,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None"]
pub fn get_challenge(this: &PublicKeyCredentialRequestOptions) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None = $1"]
pub fn set_challenge(this: &PublicKeyCredentialRequestOptions, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None"]
pub fn get_extensions(this: &PublicKeyCredentialRequestOptions,) -> Option<AuthenticationExtensionsClientInputs> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None = $1"]
pub fn set_extensions(this: &PublicKeyCredentialRequestOptions, val: &AuthenticationExtensionsClientInputs,) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_hints(this: &PublicKeyCredentialRequestOptions) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_hints(this: &PublicKeyCredentialRequestOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `rpId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None"]
pub fn get_rp_id(this: &PublicKeyCredentialRequestOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rpId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None = $1"]
pub fn set_rp_id(this: &PublicKeyCredentialRequestOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None"]
pub fn get_timeout(this: &PublicKeyCredentialRequestOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None = $1"]
pub fn set_timeout(this: &PublicKeyCredentialRequestOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Get the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`, `UserVerificationRequirement`*"]
#[js::import = "None"]
pub fn get_user_verification(this: &PublicKeyCredentialRequestOptions,) -> Option<UserVerificationRequirement> { panic!("web-sys intrinsic") }
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Change the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`, `UserVerificationRequirement`*"]
#[js::import = "None = $1"]
pub fn set_user_verification(this: &PublicKeyCredentialRequestOptions, val: UserVerificationRequirement,) { panic!("web-sys intrinsic") }
impl PublicKeyCredentialRequestOptions {
    #[doc = "Construct a new `PublicKeyCredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None"]
pub fn new(challenge: &::js_sys::Object) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_challenge(challenge);
        ret
    }
    #[deprecated = "Use `set_allow_credentials()` instead."]
#[js::import = "None"]
pub fn allow_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_allow_credentials(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_attestation()` instead."]
#[js::import = "None"]
pub fn attestation(&mut self, val: &str) -> &mut Self { self.set_attestation(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_attestation_formats()` instead."]
#[js::import = "None"]
pub fn attestation_formats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_attestation_formats(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_challenge()` instead."]
#[js::import = "None"]
pub fn challenge(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_challenge(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[deprecated = "Use `set_extensions()` instead."]
#[js::import = "None"]
pub fn extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self { self.set_extensions(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_hints()` instead."]
#[js::import = "None"]
pub fn hints(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_hints(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_rp_id()` instead."]
#[js::import = "None"]
pub fn rp_id(&mut self, val: &str) -> &mut Self { self.set_rp_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_timeout()` instead."]
#[js::import = "None"]
pub fn timeout(&mut self, val: u32) -> &mut Self { self.set_timeout(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "UserVerificationRequirement")]
    #[deprecated = "Use `set_user_verification()` instead."]
#[js::import = "None"]
pub fn user_verification(&mut self, val: UserVerificationRequirement) -> &mut Self { self.set_user_verification(val) { panic!("web-sys intrinsic") }
        self
    }
}
