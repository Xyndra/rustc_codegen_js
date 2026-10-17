#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WritableStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WritableStream(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `locked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/locked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
impl WritableStream {
    #[js::import = "$0.locked"]
pub fn locked(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `new WritableStream(..)` constructor, creating a new instance of `WritableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
impl WritableStream {
    #[js::import = "new WritableStream()"]
pub fn new() -> Result<WritableStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new WritableStream(..)` constructor, creating a new instance of `WritableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
impl WritableStream {
    #[js::import = "new WritableStream($0)"]
pub fn new_with_underlying_sink(underlying_sink: &::js_sys::Object,) -> Result<WritableStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "QueuingStrategy")]
    #[doc = "The `new WritableStream(..)` constructor, creating a new instance of `WritableStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/WritableStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`, `WritableStream`*"]
impl WritableStream {
    #[js::import = "new WritableStream($0, $1)"]
pub fn new_with_underlying_sink_and_strategy(underlying_sink: &::js_sys::Object, strategy: &QueuingStrategy,) -> Result<WritableStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
impl WritableStream {
    #[js::import = "$0.abort()"]
pub fn abort(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
impl WritableStream {
    #[js::import = "$0.abort($1)"]
pub fn abort_with_reason(&self, reason: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`*"]
impl WritableStream {
    #[js::import = "$0.close()"]
pub fn close(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WritableStreamDefaultWriter")]
    #[doc = "The `getWriter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream/getWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`, `WritableStreamDefaultWriter`*"]
impl WritableStream {
    #[js::import = "$0.getWriter()"]
pub fn get_writer(&self) -> Result<WritableStreamDefaultWriter, JsValue> { panic!("web-sys intrinsic") }
}
