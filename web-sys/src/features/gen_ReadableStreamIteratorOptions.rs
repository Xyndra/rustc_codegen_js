#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamIteratorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ReadableStreamIteratorOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
#[js::import = "None"]
pub fn get_prevent_cancel(this: &ReadableStreamIteratorOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
#[js::import = "None = $1"]
pub fn set_prevent_cancel(this: &ReadableStreamIteratorOptions, val: bool) { panic!("web-sys intrinsic") }
impl ReadableStreamIteratorOptions {
    #[doc = "Construct a new `ReadableStreamIteratorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_prevent_cancel()` instead."]
#[js::import = "None"]
pub fn prevent_cancel(&mut self, val: bool) -> &mut Self { self.set_prevent_cancel(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ReadableStreamIteratorOptions {
    fn default() -> Self {
        Self::new()
    }
}
