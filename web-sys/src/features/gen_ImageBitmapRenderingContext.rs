#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageBitmapRenderingContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapRenderingContext`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ImageBitmapRenderingContext(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `transferFromImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferFromImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*"]
impl ImageBitmapRenderingContext {
    #[js::import = "$0.transferFromImageBitmap($1)"]
pub fn transfer_from_image_bitmap(&self, bitmap: &ImageBitmap) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `transferImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmapRenderingContext/transferImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapRenderingContext`*"]
impl ImageBitmapRenderingContext {
    #[js::import = "$0.transferImageBitmap($1)"]
pub fn transfer_image_bitmap(&self, bitmap: &ImageBitmap) { panic!("web-sys intrinsic") }
}
