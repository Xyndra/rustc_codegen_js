#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `L10nValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
#[js::opaque]
#[derive(Copy)]
pub struct L10nValue(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
#[js::import = "None"]
pub fn get_attributes(this: &L10nValue) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
#[js::import = "None = $1"]
pub fn set_attributes(this: &L10nValue, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
#[js::import = "None"]
pub fn get_value(this: &L10nValue) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
#[js::import = "None = $1"]
pub fn set_value(this: &L10nValue, val: Option<&str>) { panic!("web-sys intrinsic") }
impl L10nValue {
    #[doc = "Construct a new `L10nValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_attributes()` instead."]
#[js::import = "None"]
pub fn attributes(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self { self.set_attributes(val.unwrap_or(&::wasm_bindgen::JsValue::NULL)) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_value()` instead."]
#[js::import = "None"]
pub fn value(&mut self, val: Option<&str>) -> &mut Self { self.set_value(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for L10nValue {
    fn default() -> Self {
        Self::new()
    }
}
