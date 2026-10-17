#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasCaptureMediaStreamTrack` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStreamTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasCaptureMediaStreamTrack`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CanvasCaptureMediaStreamTrack(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStreamTrack/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasCaptureMediaStreamTrack`, `HtmlCanvasElement`*"]
impl CanvasCaptureMediaStreamTrack {
    #[js::import = "$0.canvas"]
pub fn canvas(&self) -> HtmlCanvasElement { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStreamTrack/requestFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasCaptureMediaStreamTrack`*"]
impl CanvasCaptureMediaStreamTrack {
    #[js::import = "$0.requestFrame()"]
pub fn request_frame(&self) { panic!("web-sys intrinsic") }
}
