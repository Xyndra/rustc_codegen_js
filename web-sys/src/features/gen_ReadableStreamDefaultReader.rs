#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamDefaultReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ReadableStreamDefaultReader(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
impl ReadableStreamDefaultReader {
    #[js::import = "$0.closed"]
pub fn closed(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableStream")]
    #[doc = "The `new ReadableStreamDefaultReader(..)` constructor, creating a new instance of `ReadableStreamDefaultReader`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/ReadableStreamDefaultReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableStreamDefaultReader`*"]
impl ReadableStreamDefaultReader {
    #[js::import = "new ReadableStreamDefaultReader($0)"]
pub fn new(stream: &ReadableStream) -> Result<ReadableStreamDefaultReader, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
impl ReadableStreamDefaultReader {
    #[js::import = "$0.read()"]
pub fn read(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/releaseLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
impl ReadableStreamDefaultReader {
    #[js::import = "$0.releaseLock()"]
pub fn release_lock(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
impl ReadableStreamDefaultReader {
    #[js::import = "$0.cancel()"]
pub fn cancel(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultReader`*"]
impl ReadableStreamDefaultReader {
    #[js::import = "$0.cancel($1)"]
pub fn cancel_with_reason(&self, reason: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
