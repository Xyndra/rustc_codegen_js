#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageBitmap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ImageBitmap(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`*"]
impl ImageBitmap {
    #[js::import = "$0.width"]
pub fn width(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`*"]
impl ImageBitmap {
    #[js::import = "$0.height"]
pub fn height(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageBitmap/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`*"]
impl ImageBitmap {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
