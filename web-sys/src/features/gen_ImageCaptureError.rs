#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCaptureError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ImageCaptureError(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
impl ImageCaptureError {
    #[js::import = "$0.code"]
pub fn code(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageCaptureError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
impl ImageCaptureError {
    #[js::import = "$0.message"]
pub fn message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
impl ImageCaptureError {
    #[doc = "The `ImageCaptureError.FRAME_GRAB_ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub const FRAME_GRAB_ERROR: u16 = 1u64 as u16;
    #[doc = "The `ImageCaptureError.SETTINGS_ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub const SETTINGS_ERROR: u16 = 2u64 as u16;
    #[doc = "The `ImageCaptureError.PHOTO_ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub const PHOTO_ERROR: u16 = 3u64 as u16;
    #[doc = "The `ImageCaptureError.ERROR_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`*"]
    pub const ERROR_UNKNOWN: u16 = 4u64 as u16;
}
