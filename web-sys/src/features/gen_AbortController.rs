#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AbortController` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AbortController(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AbortSignal")]
    #[doc = "Getter for the `signal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/signal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`, `AbortSignal`*"]
impl AbortController {
    #[js::import = "$0.signal"]
pub fn signal(&self) -> AbortSignal { panic!("web-sys intrinsic") }
}
    #[doc = "The `new AbortController(..)` constructor, creating a new instance of `AbortController`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/AbortController)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`*"]
impl AbortController {
    #[js::import = "new AbortController()"]
pub fn new() -> Result<AbortController, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`*"]
impl AbortController {
    #[js::import = "$0.abort()"]
pub fn abort(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortController`*"]
impl AbortController {
    #[js::import = "$0.abort($1)"]
pub fn abort_with_reason(&self, reason: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
