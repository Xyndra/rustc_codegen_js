#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HkdfParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HkdfParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::import = "None"]
pub fn get_name(this: &HkdfParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &HkdfParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::import = "None"]
pub fn get_hash(this: &HkdfParams) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::import = "None = $1"]
pub fn set_hash(this: &HkdfParams, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::import = "None"]
pub fn get_info(this: &HkdfParams) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::import = "None = $1"]
pub fn set_info(this: &HkdfParams, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[doc = "Get the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::import = "None"]
pub fn get_salt(this: &HkdfParams) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::import = "None = $1"]
pub fn set_salt(this: &HkdfParams, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
impl HkdfParams {
    #[doc = "Construct a new `HkdfParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
#[js::import = "None"]
pub fn new(name: &str, hash: &::wasm_bindgen::JsValue, info: &::js_sys::Object, salt: &::js_sys::Object,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_hash(hash);
        ret.set_info(info);
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
    #[deprecated = "Use `set_info()` instead."]
#[js::import = "None"]
pub fn info(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_info(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_salt()` instead."]
#[js::import = "None"]
pub fn salt(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_salt(val) { panic!("web-sys intrinsic") }
        self
    }
}
