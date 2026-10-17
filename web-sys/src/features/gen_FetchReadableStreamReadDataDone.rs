#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchReadableStreamReadDataDone` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FetchReadableStreamReadDataDone(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
#[js::import = "None"]
pub fn get_done(this: &FetchReadableStreamReadDataDone) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
#[js::import = "None = $1"]
pub fn set_done(this: &FetchReadableStreamReadDataDone, val: bool) { panic!("web-sys intrinsic") }
impl FetchReadableStreamReadDataDone {
    #[doc = "Construct a new `FetchReadableStreamReadDataDone`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_done()` instead."]
#[js::import = "None"]
pub fn done(&mut self, val: bool) -> &mut Self { self.set_done(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FetchReadableStreamReadDataDone {
    fn default() -> Self {
        Self::new()
    }
}
