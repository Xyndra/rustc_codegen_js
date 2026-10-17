#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ReadableStream(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `locked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/locked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "$0.locked"]
pub fn locked(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ReadableStream(..)` constructor, creating a new instance of `ReadableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/ReadableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "new ReadableStream()"]
pub fn new() -> Result<ReadableStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ReadableStream(..)` constructor, creating a new instance of `ReadableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/ReadableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "new ReadableStream($0)"]
pub fn new_with_underlying_source(underlying_source: &::js_sys::Object,) -> Result<ReadableStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "QueuingStrategy")]
    #[doc = "The `new ReadableStream(..)` constructor, creating a new instance of `ReadableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/ReadableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`, `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "new ReadableStream($0, $1)"]
pub fn new_with_underlying_source_and_strategy(underlying_source: &::js_sys::Object, strategy: &QueuingStrategy,) -> Result<ReadableStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "$0.cancel()"]
pub fn cancel(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "$0.cancel($1)"]
pub fn cancel_with_reason(&self, reason: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::AsyncIterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `getReader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "$0.getReader()"]
pub fn get_reader(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableStreamGetReaderOptions")]
    #[doc = "The `getReader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/getReader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableStreamGetReaderOptions`*"]
impl ReadableStream {
    #[js::import = "$0.getReader($1)"]
pub fn get_reader_with_options(&self, options: &ReadableStreamGetReaderOptions) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::AsyncIterator { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableWritablePair")]
    #[doc = "The `pipeThrough()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
impl ReadableStream {
    #[js::import = "$0.pipeThrough($1)"]
pub fn pipe_through(&self, transform: &ReadableWritablePair) -> ReadableStream { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ReadableWritablePair", feature = "StreamPipeOptions",))]
    #[doc = "The `pipeThrough()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeThrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`, `StreamPipeOptions`*"]
impl ReadableStream {
    #[js::import = "$0.pipeThrough($1, $2)"]
pub fn pipe_through_with_options(&self, transform: &ReadableWritablePair, options: &StreamPipeOptions) -> ReadableStream { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WritableStream")]
    #[doc = "The `pipeTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `WritableStream`*"]
impl ReadableStream {
    #[js::import = "$0.pipeTo($1)"]
pub fn pipe_to(&self, destination: &WritableStream) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "StreamPipeOptions", feature = "WritableStream",))]
    #[doc = "The `pipeTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/pipeTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `StreamPipeOptions`, `WritableStream`*"]
impl ReadableStream {
    #[js::import = "$0.pipeTo($1, $2)"]
pub fn pipe_to_with_options(&self, destination: &WritableStream, options: &StreamPipeOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `tee()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/tee)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "$0.tee()"]
pub fn tee(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`*"]
impl ReadableStream {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::AsyncIterator { panic!("web-sys intrinsic") }
}
