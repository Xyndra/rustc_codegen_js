#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageCaptureErrorEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ImageCaptureErrorEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &ImageCaptureErrorEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &ImageCaptureErrorEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &ImageCaptureErrorEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &ImageCaptureErrorEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &ImageCaptureErrorEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &ImageCaptureErrorEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ImageCaptureError")]
    #[doc = "Get the `imageCaptureError` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEventInit`*"]
#[js::import = "None"]
pub fn get_image_capture_error(this: &ImageCaptureErrorEventInit) -> Option<ImageCaptureError> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ImageCaptureError")]
    #[doc = "Change the `imageCaptureError` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureError`, `ImageCaptureErrorEventInit`*"]
#[js::import = "None = $1"]
pub fn set_image_capture_error(this: &ImageCaptureErrorEventInit, val: Option<&ImageCaptureError>,) { panic!("web-sys intrinsic") }
impl ImageCaptureErrorEventInit {
    #[doc = "Construct a new `ImageCaptureErrorEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageCaptureErrorEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ImageCaptureError")]
    #[deprecated = "Use `set_image_capture_error()` instead."]
#[js::import = "None"]
pub fn image_capture_error(&mut self, val: Option<&ImageCaptureError>) -> &mut Self { self.set_image_capture_error(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ImageCaptureErrorEventInit {
    fn default() -> Self {
        Self::new()
    }
}
