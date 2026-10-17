#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IterableKeyOrValueResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IterableKeyOrValueResult(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
#[js::import = "None"]
pub fn get_done(this: &IterableKeyOrValueResult) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
#[js::import = "None = $1"]
pub fn set_done(this: &IterableKeyOrValueResult, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
#[js::import = "None"]
pub fn get_value(this: &IterableKeyOrValueResult) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
#[js::import = "None = $1"]
pub fn set_value(this: &IterableKeyOrValueResult, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl IterableKeyOrValueResult {
    #[doc = "Construct a new `IterableKeyOrValueResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_done()` instead."]
#[js::import = "None"]
pub fn done(&mut self, val: bool) -> &mut Self { self.set_done(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_value()` instead."]
#[js::import = "None"]
pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_value(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for IterableKeyOrValueResult {
    fn default() -> Self {
        Self::new()
    }
}
