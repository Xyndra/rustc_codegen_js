#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsCacheDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DnsCacheDict(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
#[js::import = "None"]
pub fn get_entries(this: &DnsCacheDict) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
#[js::import = "None = $1"]
pub fn set_entries(this: &DnsCacheDict, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl DnsCacheDict {
    #[doc = "Construct a new `DnsCacheDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_entries()` instead."]
#[js::import = "None"]
pub fn entries(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_entries(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DnsCacheDict {
    fn default() -> Self {
        Self::new()
    }
}
