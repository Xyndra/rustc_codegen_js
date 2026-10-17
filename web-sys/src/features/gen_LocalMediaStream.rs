#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LocalMediaStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LocalMediaStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocalMediaStream`*"]
#[js::opaque]
#[derive(Copy)]
pub struct LocalMediaStream(pub ::wasm_bindgen::JsValue);
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/LocalMediaStream/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocalMediaStream`*"]
impl LocalMediaStream {
    #[js::import = "$0.stop()"]
pub fn stop(&self) { panic!("web-sys intrinsic") }
}
