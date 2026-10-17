#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WritableStreamDefaultWriter` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WritableStreamDefaultWriter(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "$0.closed"]
pub fn closed(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `desiredSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/desiredSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "$0.desiredSize"]
pub fn desired_size(&self) -> Result<Option<f64>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "$0.ready"]
pub fn ready(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WritableStream")]
    #[doc = "The `new WritableStreamDefaultWriter(..)` constructor, creating a new instance of `WritableStreamDefaultWriter`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/WritableStreamDefaultWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStream`, `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "new WritableStreamDefaultWriter($0)"]
pub fn new(stream: &WritableStream) -> Result<WritableStreamDefaultWriter, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "$0.abort()"]
pub fn abort(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "$0.abort($1)"]
pub fn abort_with_reason(&self, reason: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "$0.close()"]
pub fn close(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `releaseLock()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/releaseLock)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "$0.releaseLock()"]
pub fn release_lock(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "$0.write()"]
pub fn write(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultWriter`*"]
impl WritableStreamDefaultWriter {
    #[js::import = "$0.write($1)"]
pub fn write_with_chunk(&self, chunk: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
