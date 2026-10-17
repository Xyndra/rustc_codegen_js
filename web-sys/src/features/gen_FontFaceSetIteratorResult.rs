#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetIteratorResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FontFaceSetIteratorResult(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
#[js::import = "None"]
pub fn get_done(this: &FontFaceSetIteratorResult) -> bool { panic!("web-sys intrinsic") }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
#[js::import = "None = $1"]
pub fn set_done(this: &FontFaceSetIteratorResult, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
#[js::import = "None"]
pub fn get_value(this: &FontFaceSetIteratorResult) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
#[js::import = "None = $1"]
pub fn set_value(this: &FontFaceSetIteratorResult, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl FontFaceSetIteratorResult {
    #[doc = "Construct a new `FontFaceSetIteratorResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
#[js::import = "None"]
pub fn new(done: bool, value: &::wasm_bindgen::JsValue) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_done(done);
        ret.set_value(value);
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
