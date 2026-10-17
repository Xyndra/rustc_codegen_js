#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OffscreenCanvas` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OffscreenCanvas(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "$0.width"]
pub fn width(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "$0.width = $1"]
pub fn set_width(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "$0.height"]
pub fn height(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "$0.height = $1"]
pub fn set_height(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new OffscreenCanvas(..)` constructor, creating a new instance of `OffscreenCanvas`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/OffscreenCanvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "new OffscreenCanvas($0, $1)"]
pub fn new(width: u32, height: u32) -> Result<OffscreenCanvas, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `convertToBlob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/convertToBlob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "$0.convertToBlob()"]
pub fn convert_to_blob(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageEncodeOptions")]
    #[doc = "The `convertToBlob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/convertToBlob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageEncodeOptions`, `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "$0.convertToBlob($1)"]
pub fn convert_to_blob_with_options(&self, options: &ImageEncodeOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/getContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "$0.getContext($1)"]
pub fn get_context(&self, context_id: &str) -> Result<Option<::js_sys::Object>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/getContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "$0.getContext($1, $2)"]
pub fn get_context_with_context_options(&self, context_id: &str, context_options: &::wasm_bindgen::JsValue) -> Result<Option<::js_sys::Object>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `transferToImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas/transferToImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `OffscreenCanvas`*"]
impl OffscreenCanvas {
    #[js::import = "$0.transferToImageBitmap()"]
pub fn transfer_to_image_bitmap(&self) -> Result<ImageBitmap, JsValue> { panic!("web-sys intrinsic") }
}
