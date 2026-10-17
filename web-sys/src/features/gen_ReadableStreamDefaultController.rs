#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamDefaultController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ReadableStreamDefaultController(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `desiredSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/desiredSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
impl ReadableStreamDefaultController {
    #[js::import = "$0.desiredSize"]
pub fn desired_size(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
impl ReadableStreamDefaultController {
    #[js::import = "$0.close()"]
pub fn close(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
impl ReadableStreamDefaultController {
    #[js::import = "$0.enqueue()"]
pub fn enqueue(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
impl ReadableStreamDefaultController {
    #[js::import = "$0.enqueue($1)"]
pub fn enqueue_with_chunk(&self, chunk: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
impl ReadableStreamDefaultController {
    #[js::import = "$0.error()"]
pub fn error(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamDefaultController`*"]
impl ReadableStreamDefaultController {
    #[js::import = "$0.error($1)"]
pub fn error_with_e(&self, e: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
