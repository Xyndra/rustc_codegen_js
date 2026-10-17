#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamGetReaderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ReadableStreamGetReaderOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ReadableStreamReaderMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`, `ReadableStreamReaderMode`*"]
#[js::import = "None"]
pub fn get_mode(this: &ReadableStreamGetReaderOptions) -> Option<ReadableStreamReaderMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ReadableStreamReaderMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`, `ReadableStreamReaderMode`*"]
#[js::import = "None = $1"]
pub fn set_mode(this: &ReadableStreamGetReaderOptions, val: ReadableStreamReaderMode) { panic!("web-sys intrinsic") }
impl ReadableStreamGetReaderOptions {
    #[doc = "Construct a new `ReadableStreamGetReaderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "ReadableStreamReaderMode")]
    #[deprecated = "Use `set_mode()` instead."]
#[js::import = "None"]
pub fn mode(&mut self, val: ReadableStreamReaderMode) -> &mut Self { self.set_mode(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ReadableStreamGetReaderOptions {
    fn default() -> Self {
        Self::new()
    }
}
