#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CredentialRequestOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[doc = "Get the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None"]
pub fn get_public_key(this: &CredentialRequestOptions,) -> Option<PublicKeyCredentialRequestOptions> { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `PublicKeyCredentialRequestOptions`*"]
#[js::import = "None = $1"]
pub fn set_public_key(this: &CredentialRequestOptions, val: &PublicKeyCredentialRequestOptions) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialRequestOptions`*"]
#[js::import = "None"]
pub fn get_signal(this: &CredentialRequestOptions) -> Option<AbortSignal> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialRequestOptions`*"]
#[js::import = "None = $1"]
pub fn set_signal(this: &CredentialRequestOptions, val: &AbortSignal) { panic!("web-sys intrinsic") }
impl CredentialRequestOptions {
    #[doc = "Construct a new `CredentialRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "PublicKeyCredentialRequestOptions")]
    #[deprecated = "Use `set_public_key()` instead."]
#[js::import = "None"]
pub fn public_key(&mut self, val: &PublicKeyCredentialRequestOptions) -> &mut Self { self.set_public_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[deprecated = "Use `set_signal()` instead."]
#[js::import = "None"]
pub fn signal(&mut self, val: &AbortSignal) -> &mut Self { self.set_signal(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for CredentialRequestOptions {
    fn default() -> Self {
        Self::new()
    }
}
