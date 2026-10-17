#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasePropertyIndexedKeyframe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BasePropertyIndexedKeyframe(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
#[js::import = "None"]
pub fn get_composite(this: &BasePropertyIndexedKeyframe) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_composite(this: &BasePropertyIndexedKeyframe, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
#[js::import = "None"]
pub fn get_easing(this: &BasePropertyIndexedKeyframe) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_easing(this: &BasePropertyIndexedKeyframe, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
#[js::import = "None"]
pub fn get_offset(this: &BasePropertyIndexedKeyframe) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_offset(this: &BasePropertyIndexedKeyframe, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl BasePropertyIndexedKeyframe {
    #[doc = "Construct a new `BasePropertyIndexedKeyframe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_composite()` instead."]
#[js::import = "None"]
pub fn composite(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_composite(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_easing()` instead."]
#[js::import = "None"]
pub fn easing(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_easing(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_offset()` instead."]
#[js::import = "None"]
pub fn offset(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_offset(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BasePropertyIndexedKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
