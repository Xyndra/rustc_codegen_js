#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCaptureErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ImageCaptureErrorEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ImageCaptureError")]
    #[doc = "Getter for the `imageCaptureError` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/imageCaptureError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEvent`*"]
impl ImageCaptureErrorEvent {
    #[js::import = "$0.imageCaptureError"]
pub fn image_capture_error(&self) -> Option<ImageCaptureError> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ImageCaptureErrorEvent(..)` constructor, creating a new instance of `ImageCaptureErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/ImageCaptureErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`*"]
impl ImageCaptureErrorEvent {
    #[js::import = "new ImageCaptureErrorEvent($0)"]
pub fn new(type_: &str) -> Result<ImageCaptureErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageCaptureErrorEventInit")]
    #[doc = "The `new ImageCaptureErrorEvent(..)` constructor, creating a new instance of `ImageCaptureErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureErrorEvent/ImageCaptureErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEvent`, `ImageCaptureErrorEventInit`*"]
impl ImageCaptureErrorEvent {
    #[js::import = "new ImageCaptureErrorEvent($0, $1)"]
pub fn new_with_image_capture_error_init_dict(type_: &str, image_capture_error_init_dict: &ImageCaptureErrorEventInit,) -> Result<ImageCaptureErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}
