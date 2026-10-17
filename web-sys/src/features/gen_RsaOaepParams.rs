#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaOaepParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RsaOaepParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
#[js::import = "None"]
pub fn get_name(this: &RsaOaepParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &RsaOaepParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
#[js::import = "None"]
pub fn get_label(this: &RsaOaepParams) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
#[js::import = "None = $1"]
pub fn set_label(this: &RsaOaepParams, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
impl RsaOaepParams {
    #[doc = "Construct a new `RsaOaepParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOaepParams`*"]
#[js::import = "None"]
pub fn new(name: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_label()` instead."]
#[js::import = "None"]
pub fn label(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_label(val) { panic!("web-sys intrinsic") }
        self
    }
}
