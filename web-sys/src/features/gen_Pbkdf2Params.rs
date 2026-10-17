#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Pbkdf2Params` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Pbkdf2Params(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::import = "None"]
pub fn get_name(this: &Pbkdf2Params) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &Pbkdf2Params, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::import = "None"]
pub fn get_hash(this: &Pbkdf2Params) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::import = "None = $1"]
pub fn set_hash(this: &Pbkdf2Params, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::import = "None"]
pub fn get_iterations(this: &Pbkdf2Params) -> u32 { panic!("web-sys intrinsic") }
    #[doc = "Change the `iterations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::import = "None = $1"]
pub fn set_iterations(this: &Pbkdf2Params, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::import = "None"]
pub fn get_salt(this: &Pbkdf2Params) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::import = "None = $1"]
pub fn set_salt(this: &Pbkdf2Params, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
impl Pbkdf2Params {
    #[doc = "Construct a new `Pbkdf2Params`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Pbkdf2Params`*"]
#[js::import = "None"]
pub fn new(name: &str, hash: &::wasm_bindgen::JsValue, iterations: u32, salt: &::js_sys::Object,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_hash(hash);
        ret.set_iterations(iterations);
        ret.set_salt(salt);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_hash()` instead."]
#[js::import = "None"]
pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_hash(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_iterations()` instead."]
#[js::import = "None"]
pub fn iterations(&mut self, val: u32) -> &mut Self { self.set_iterations(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_salt()` instead."]
#[js::import = "None"]
pub fn salt(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_salt(val) { panic!("web-sys intrinsic") }
        self
    }
}
