#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamByobReader` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ReadableStreamByobReader(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
impl ReadableStreamByobReader {
    #[js::import = "$0.closed"]
pub fn closed(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableStream")]
    #[doc = "The `new ReadableStreamByobReader(..)` constructor, creating a new instance of `ReadableStreamByobReader`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/ReadableStreamBYOBReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableStreamByobReader`*"]
impl ReadableStreamByobReader {
    #[js::import = "new ReadableStreamBYOBReader($0)"]
pub fn new(stream: &ReadableStream) -> Result<ReadableStreamByobReader, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
impl ReadableStreamByobReader {
    #[js::import = "$0.read($1)"]
pub fn read_with_array_buffer_view(&self, view: &::js_sys::Object) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/releaseLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
impl ReadableStreamByobReader {
    #[js::import = "$0.releaseLock()"]
pub fn release_lock(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
impl ReadableStreamByobReader {
    #[js::import = "$0.cancel()"]
pub fn cancel(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobReader`*"]
impl ReadableStreamByobReader {
    #[js::import = "$0.cancel($1)"]
pub fn cancel_with_reason(&self, reason: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
