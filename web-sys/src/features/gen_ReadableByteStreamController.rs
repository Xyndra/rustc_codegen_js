#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableByteStreamController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ReadableByteStreamController(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ReadableStreamByobRequest")]
    #[doc = "Getter for the `byobRequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/byobRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`, `ReadableStreamByobRequest`*"]
impl ReadableByteStreamController {
    #[js::import = "$0.byobRequest"]
pub fn byob_request(&self) -> Option<ReadableStreamByobRequest> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `desiredSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/desiredSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
impl ReadableByteStreamController {
    #[js::import = "$0.desiredSize"]
pub fn desired_size(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
impl ReadableByteStreamController {
    #[js::import = "$0.close()"]
pub fn close(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
impl ReadableByteStreamController {
    #[js::import = "$0.enqueue($1)"]
pub fn enqueue_with_array_buffer_view(&self, chunk: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
impl ReadableByteStreamController {
    #[js::import = "$0.enqueue($1)"]
pub fn enqueue_with_u8_array(&self, chunk: &mut [u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
impl ReadableByteStreamController {
    #[js::import = "$0.enqueue($1)"]
pub fn enqueue_with_js_u8_array(&self, chunk: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
impl ReadableByteStreamController {
    #[js::import = "$0.error()"]
pub fn error(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableByteStreamController`*"]
impl ReadableByteStreamController {
    #[js::import = "$0.error($1)"]
pub fn error_with_e(&self, e: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
