#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HmacKeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HmacKeyAlgorithm(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
#[js::import = "None"]
pub fn get_name(this: &HmacKeyAlgorithm) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &HmacKeyAlgorithm, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
#[js::import = "None"]
pub fn get_hash(this: &HmacKeyAlgorithm) -> KeyAlgorithm { panic!("web-sys intrinsic") }
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
#[js::import = "None = $1"]
pub fn set_hash(this: &HmacKeyAlgorithm, val: &KeyAlgorithm) { panic!("web-sys intrinsic") }
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
#[js::import = "None"]
pub fn get_length(this: &HmacKeyAlgorithm) -> u32 { panic!("web-sys intrinsic") }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
#[js::import = "None = $1"]
pub fn set_length(this: &HmacKeyAlgorithm, val: u32) { panic!("web-sys intrinsic") }
impl HmacKeyAlgorithm {
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Construct a new `HmacKeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
#[js::import = "None"]
pub fn new(name: &str, hash: &KeyAlgorithm, length: u32) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_hash(hash);
        ret.set_length(length);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "KeyAlgorithm")]
    #[deprecated = "Use `set_hash()` instead."]
#[js::import = "None"]
pub fn hash(&mut self, val: &KeyAlgorithm) -> &mut Self { self.set_hash(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_length()` instead."]
#[js::import = "None"]
pub fn length(&mut self, val: u32) -> &mut Self { self.set_length(val) { panic!("web-sys intrinsic") }
        self
    }
}
