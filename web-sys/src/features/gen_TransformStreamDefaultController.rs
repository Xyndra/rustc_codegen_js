#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TransformStreamDefaultController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TransformStreamDefaultController(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `desiredSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/desiredSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
impl TransformStreamDefaultController {
    #[js::import = "$0.desiredSize"]
pub fn desired_size(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
impl TransformStreamDefaultController {
    #[js::import = "$0.enqueue()"]
pub fn enqueue(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `enqueue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/enqueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
impl TransformStreamDefaultController {
    #[js::import = "$0.enqueue($1)"]
pub fn enqueue_with_chunk(&self, chunk: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
impl TransformStreamDefaultController {
    #[js::import = "$0.error()"]
pub fn error(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
impl TransformStreamDefaultController {
    #[js::import = "$0.error($1)"]
pub fn error_with_reason(&self, reason: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "The `terminate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/terminate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransformStreamDefaultController`*"]
impl TransformStreamDefaultController {
    #[js::import = "$0.terminate()"]
pub fn terminate(&self) { panic!("web-sys intrinsic") }
}
