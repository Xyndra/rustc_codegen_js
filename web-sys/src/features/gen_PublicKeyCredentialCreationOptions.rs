#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialCreationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PublicKeyCredentialCreationOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AttestationConveyancePreference")]
    #[doc = "Get the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttestationConveyancePreference`, `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None"]
pub fn get_attestation(this: &PublicKeyCredentialCreationOptions,) -> Option<AttestationConveyancePreference> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AttestationConveyancePreference")]
    #[doc = "Change the `attestation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttestationConveyancePreference`, `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_attestation(this: &PublicKeyCredentialCreationOptions, val: AttestationConveyancePreference,) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_attestation_formats(this: &PublicKeyCredentialCreationOptions,) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attestationFormats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_attestation_formats(this: &PublicKeyCredentialCreationOptions, val: &::wasm_bindgen::JsValue,) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Get the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None"]
pub fn get_authenticator_selection(this: &PublicKeyCredentialCreationOptions,) -> Option<AuthenticatorSelectionCriteria> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[doc = "Change the `authenticatorSelection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_authenticator_selection(this: &PublicKeyCredentialCreationOptions, val: &AuthenticatorSelectionCriteria,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None"]
pub fn get_challenge(this: &PublicKeyCredentialCreationOptions) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_challenge(this: &PublicKeyCredentialCreationOptions, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[doc = "Get the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None"]
pub fn get_exclude_credentials(this: &PublicKeyCredentialCreationOptions,) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `excludeCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_exclude_credentials(this: &PublicKeyCredentialCreationOptions, val: &::wasm_bindgen::JsValue,) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None"]
pub fn get_extensions(this: &PublicKeyCredentialCreationOptions,) -> Option<AuthenticationExtensionsClientInputs> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_extensions(this: &PublicKeyCredentialCreationOptions, val: &AuthenticationExtensionsClientInputs,) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_hints(this: &PublicKeyCredentialCreationOptions) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `hints` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_hints(this: &PublicKeyCredentialCreationOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None"]
pub fn get_pub_key_cred_params(this: &PublicKeyCredentialCreationOptions) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[doc = "Change the `pubKeyCredParams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_pub_key_cred_params(this: &PublicKeyCredentialCreationOptions, val: &::wasm_bindgen::JsValue,) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Get the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialRpEntity`*"]
#[js::import = "None"]
pub fn get_rp(this: &PublicKeyCredentialCreationOptions) -> PublicKeyCredentialRpEntity { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[doc = "Change the `rp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialRpEntity`*"]
#[js::import = "None = $1"]
pub fn set_rp(this: &PublicKeyCredentialCreationOptions, val: &PublicKeyCredentialRpEntity) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None"]
pub fn get_timeout(this: &PublicKeyCredentialCreationOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_timeout(this: &PublicKeyCredentialCreationOptions, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[doc = "Get the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialUserEntity`*"]
#[js::import = "None"]
pub fn get_user(this: &PublicKeyCredentialCreationOptions) -> PublicKeyCredentialUserEntity { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[doc = "Change the `user` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialUserEntity`*"]
#[js::import = "None = $1"]
pub fn set_user(this: &PublicKeyCredentialCreationOptions, val: &PublicKeyCredentialUserEntity) { panic!("web-sys intrinsic") }
impl PublicKeyCredentialCreationOptions {
    #[cfg(all(
        feature = "PublicKeyCredentialRpEntity",
        feature = "PublicKeyCredentialUserEntity",
    ))]
    #[doc = "Construct a new `PublicKeyCredentialCreationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialCreationOptions`, `PublicKeyCredentialRpEntity`, `PublicKeyCredentialUserEntity`*"]
#[js::import = "None"]
pub fn new(challenge: &::js_sys::Object, pub_key_cred_params: &::wasm_bindgen::JsValue, rp: &PublicKeyCredentialRpEntity, user: &PublicKeyCredentialUserEntity,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_challenge(challenge);
        ret.set_pub_key_cred_params(pub_key_cred_params);
        ret.set_rp(rp);
        ret.set_user(user);
        ret
    }
    #[cfg(feature = "AttestationConveyancePreference")]
    #[deprecated = "Use `set_attestation()` instead."]
#[js::import = "None"]
pub fn attestation(&mut self, val: AttestationConveyancePreference) -> &mut Self { self.set_attestation(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_attestation_formats()` instead."]
#[js::import = "None"]
pub fn attestation_formats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_attestation_formats(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AuthenticatorSelectionCriteria")]
    #[deprecated = "Use `set_authenticator_selection()` instead."]
#[js::import = "None"]
pub fn authenticator_selection(&mut self, val: &AuthenticatorSelectionCriteria) -> &mut Self { self.set_authenticator_selection(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_challenge()` instead."]
#[js::import = "None"]
pub fn challenge(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_challenge(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_exclude_credentials()` instead."]
#[js::import = "None"]
pub fn exclude_credentials(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_exclude_credentials(val) { panic!("web-sys intrinsic") }
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
    #[deprecated = "Use `set_pub_key_cred_params()` instead."]
#[js::import = "None"]
pub fn pub_key_cred_params(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_pub_key_cred_params(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "PublicKeyCredentialRpEntity")]
    #[deprecated = "Use `set_rp()` instead."]
#[js::import = "None"]
pub fn rp(&mut self, val: &PublicKeyCredentialRpEntity) -> &mut Self { self.set_rp(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_timeout()` instead."]
#[js::import = "None"]
pub fn timeout(&mut self, val: u32) -> &mut Self { self.set_timeout(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "PublicKeyCredentialUserEntity")]
    #[deprecated = "Use `set_user()` instead."]
#[js::import = "None"]
pub fn user(&mut self, val: &PublicKeyCredentialUserEntity) -> &mut Self { self.set_user(val) { panic!("web-sys intrinsic") }
        self
    }
}
