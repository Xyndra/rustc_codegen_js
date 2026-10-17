#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DhKeyDeriveParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DhKeyDeriveParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
#[js::import = "None"]
pub fn get_name(this: &DhKeyDeriveParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DhKeyDeriveParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &DhKeyDeriveParams, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Get the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
#[js::import = "None"]
pub fn get_public(this: &DhKeyDeriveParams) -> CryptoKey { panic!("web-sys intrinsic") }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
#[js::import = "None = $1"]
pub fn set_public(this: &DhKeyDeriveParams, val: &CryptoKey) { panic!("web-sys intrinsic") }
impl DhKeyDeriveParams {
    #[cfg(feature = "CryptoKey")]
    #[doc = "Construct a new `DhKeyDeriveParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `DhKeyDeriveParams`*"]
#[js::import = "None"]
pub fn new(name: &str, public: &CryptoKey) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_public(public);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "CryptoKey")]
    #[deprecated = "Use `set_public()` instead."]
#[js::import = "None"]
pub fn public(&mut self, val: &CryptoKey) -> &mut Self { self.set_public(val) { panic!("web-sys intrinsic") }
        self
    }
}
