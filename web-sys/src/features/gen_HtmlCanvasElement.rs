#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlCanvasElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlCanvasElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.width = $1"]
pub fn set_width(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.height = $1"]
pub fn set_height(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStream")]
    #[doc = "The `captureStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/captureStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `MediaStream`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.captureStream()"]
pub fn capture_stream(&self) -> Result<MediaStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStream")]
    #[doc = "The `captureStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/captureStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `MediaStream`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.captureStream($1)"]
pub fn capture_stream_with_frame_request_rate(&self, frame_request_rate: f64) -> Result<MediaStream, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.getContext($1)"]
pub fn get_context(&self, context_id: &str) -> Result<Option<::js_sys::Object>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/getContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.getContext($1, $2)"]
pub fn get_context_with_context_options(&self, context_id: &str, context_options: &::wasm_bindgen::JsValue) -> Result<Option<::js_sys::Object>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toBlob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.toBlob($1)"]
pub fn to_blob(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toBlob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.toBlob($1, $2)"]
pub fn to_blob_with_type(&self, callback: &::js_sys::Function, type_: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toBlob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toBlob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.toBlob($1, $2, $3)"]
pub fn to_blob_with_type_and_encoder_options(&self, callback: &::js_sys::Function, type_: &str, encoder_options: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.toDataURL()"]
pub fn to_data_url(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.toDataURL($1)"]
pub fn to_data_url_with_type(&self, type_: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toDataURL()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/toDataURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.toDataURL($1, $2)"]
pub fn to_data_url_with_type_and_encoder_options(&self, type_: &str, encoder_options: &::wasm_bindgen::JsValue) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "The `transferControlToOffscreen()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/transferControlToOffscreen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `OffscreenCanvas`*"]
impl HtmlCanvasElement {
    #[js::import = "$0.transferControlToOffscreen()"]
pub fn transfer_control_to_offscreen(&self) -> Result<OffscreenCanvas, JsValue> { panic!("web-sys intrinsic") }
}
