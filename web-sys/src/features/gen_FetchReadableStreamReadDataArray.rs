#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchReadableStreamReadDataArray` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FetchReadableStreamReadDataArray(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
#[js::import = "None"]
pub fn get_value(this: &FetchReadableStreamReadDataArray) -> Option<::alloc::vec::Vec<u8>> { panic!("web-sys intrinsic") }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
#[js::import = "None = $1"]
pub fn set_value(this: &FetchReadableStreamReadDataArray, val: &::js_sys::Uint8Array) { panic!("web-sys intrinsic") }
impl FetchReadableStreamReadDataArray {
    #[doc = "Construct a new `FetchReadableStreamReadDataArray`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataArray`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_value()` instead."]
#[js::import = "None"]
pub fn value(&mut self, val: &::js_sys::Uint8Array) -> &mut Self { self.set_value(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FetchReadableStreamReadDataArray {
    fn default() -> Self {
        Self::new()
    }
}
