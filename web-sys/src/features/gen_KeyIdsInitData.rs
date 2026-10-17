#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `KeyIdsInitData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
#[js::opaque]
#[derive(Copy)]
pub struct KeyIdsInitData(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `kids` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
#[js::import = "None"]
pub fn get_kids(this: &KeyIdsInitData) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[doc = "Change the `kids` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
#[js::import = "None = $1"]
pub fn set_kids(this: &KeyIdsInitData, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl KeyIdsInitData {
    #[doc = "Construct a new `KeyIdsInitData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `KeyIdsInitData`*"]
#[js::import = "None"]
pub fn new(kids: &::wasm_bindgen::JsValue) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_kids(kids);
        ret
    }
    #[deprecated = "Use `set_kids()` instead."]
#[js::import = "None"]
pub fn kids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_kids(val) { panic!("web-sys intrinsic") }
        self
    }
}
