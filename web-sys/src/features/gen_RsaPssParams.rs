#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaPssParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RsaPssParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
#[js::import = "None"]
pub fn get_name(this: &RsaPssParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &RsaPssParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `saltLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
#[js::import = "None"]
pub fn get_salt_length(this: &RsaPssParams) -> u32 { panic!("web-sys intrinsic") }
    #[doc = "Change the `saltLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
#[js::import = "None = $1"]
pub fn set_salt_length(this: &RsaPssParams, val: u32) { panic!("web-sys intrinsic") }
impl RsaPssParams {
    #[doc = "Construct a new `RsaPssParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
#[js::import = "None"]
pub fn new(name: &str, salt_length: u32) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_salt_length(salt_length);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_salt_length()` instead."]
#[js::import = "None"]
pub fn salt_length(&mut self, val: u32) -> &mut Self { self.set_salt_length(val) { panic!("web-sys intrinsic") }
        self
    }
}
