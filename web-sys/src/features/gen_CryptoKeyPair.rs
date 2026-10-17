#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CryptoKeyPair` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKeyPair`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CryptoKeyPair(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CryptoKey")]
    #[doc = "Get the `privateKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
#[js::import = "None"]
pub fn get_private_key(this: &CryptoKeyPair) -> CryptoKey { panic!("web-sys intrinsic") }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `privateKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
#[js::import = "None = $1"]
pub fn set_private_key(this: &CryptoKeyPair, val: &CryptoKey) { panic!("web-sys intrinsic") }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Get the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
#[js::import = "None"]
pub fn get_public_key(this: &CryptoKeyPair) -> CryptoKey { panic!("web-sys intrinsic") }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
#[js::import = "None = $1"]
pub fn set_public_key(this: &CryptoKeyPair, val: &CryptoKey) { panic!("web-sys intrinsic") }
impl CryptoKeyPair {
    #[cfg(feature = "CryptoKey")]
    #[doc = "Construct a new `CryptoKeyPair`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `CryptoKeyPair`*"]
#[js::import = "None"]
pub fn new(private_key: &CryptoKey, public_key: &CryptoKey) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_private_key(private_key);
        ret.set_public_key(public_key);
        ret
    }
    #[cfg(feature = "CryptoKey")]
    #[deprecated = "Use `set_private_key()` instead."]
#[js::import = "None"]
pub fn private_key(&mut self, val: &CryptoKey) -> &mut Self { self.set_private_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "CryptoKey")]
    #[deprecated = "Use `set_public_key()` instead."]
#[js::import = "None"]
pub fn public_key(&mut self, val: &CryptoKey) -> &mut Self { self.set_public_key(val) { panic!("web-sys intrinsic") }
        self
    }
}
