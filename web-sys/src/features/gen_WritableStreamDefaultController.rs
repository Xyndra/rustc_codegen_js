#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WritableStreamDefaultController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultController`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WritableStreamDefaultController(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AbortSignal")]
    #[doc = "Getter for the `signal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController/signal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `WritableStreamDefaultController`*"]
impl WritableStreamDefaultController {
    #[js::import = "$0.signal"]
pub fn signal(&self) -> AbortSignal { panic!("web-sys intrinsic") }
}
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultController`*"]
impl WritableStreamDefaultController {
    #[js::import = "$0.error()"]
pub fn error(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WritableStreamDefaultController`*"]
impl WritableStreamDefaultController {
    #[js::import = "$0.error($1)"]
pub fn error_with_e(&self, e: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
