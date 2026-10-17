#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ImageData(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
impl ImageData {
    #[js::import = "$0.width"]
pub fn width(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
impl ImageData {
    #[js::import = "$0.height"]
pub fn height(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
impl ImageData {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::wasm_bindgen::Clamped<::alloc::vec::Vec<u8>> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
impl ImageData {
    #[js::import = "new ImageData($0, $1)"]
pub fn new_with_sw(sw: u32, sh: u32) -> Result<ImageData, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
impl ImageData {
    #[js::import = "new ImageData($0, $1)"]
pub fn new_with_u8_clamped_array(data: ::wasm_bindgen::Clamped<&[u8]>, sw: u32,) -> Result<ImageData, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
impl ImageData {
    #[js::import = "new ImageData($0, $1)"]
pub fn new_with_js_u8_clamped_array(data: &::js_sys::Uint8ClampedArray, sw: u32,) -> Result<ImageData, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
impl ImageData {
    #[js::import = "new ImageData($0, $1, $2)"]
pub fn new_with_u8_clamped_array_and_sh(data: ::wasm_bindgen::Clamped<&[u8]>, sw: u32, sh: u32,) -> Result<ImageData, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ImageData(..)` constructor, creating a new instance of `ImageData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`*"]
impl ImageData {
    #[js::import = "new ImageData($0, $1, $2)"]
pub fn new_with_js_u8_clamped_array_and_sh(data: &::js_sys::Uint8ClampedArray, sw: u32, sh: u32,) -> Result<ImageData, JsValue> { panic!("web-sys intrinsic") }
}
