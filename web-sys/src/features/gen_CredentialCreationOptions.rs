#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialCreationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CredentialCreationOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    #[doc = "Get the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`, `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None"]
pub fn get_public_key(this: &CredentialCreationOptions,) -> Option<PublicKeyCredentialCreationOptions> { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`, `PublicKeyCredentialCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_public_key(this: &CredentialCreationOptions, val: &PublicKeyCredentialCreationOptions,) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialCreationOptions`*"]
#[js::import = "None"]
pub fn get_signal(this: &CredentialCreationOptions) -> Option<AbortSignal> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_signal(this: &CredentialCreationOptions, val: &AbortSignal) { panic!("web-sys intrinsic") }
impl CredentialCreationOptions {
    #[doc = "Construct a new `CredentialCreationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    #[deprecated = "Use `set_public_key()` instead."]
#[js::import = "None"]
pub fn public_key(&mut self, val: &PublicKeyCredentialCreationOptions) -> &mut Self { self.set_public_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[deprecated = "Use `set_signal()` instead."]
#[js::import = "None"]
pub fn signal(&mut self, val: &AbortSignal) -> &mut Self { self.set_signal(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for CredentialCreationOptions {
    fn default() -> Self {
        Self::new()
    }
}
