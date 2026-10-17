#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamByobRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ReadableStreamByobRequest(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `view` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/view)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
impl ReadableStreamByobRequest {
    #[js::import = "$0.view"]
pub fn view(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "The `respond()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respond)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
impl ReadableStreamByobRequest {
    #[js::import = "$0.respond($1)"]
pub fn respond_with_u32(&self, bytes_written: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `respond()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respond)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
impl ReadableStreamByobRequest {
    #[js::import = "$0.respond($1)"]
pub fn respond_with_f64(&self, bytes_written: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `respondWithNewView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respondWithNewView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
impl ReadableStreamByobRequest {
    #[js::import = "$0.respondWithNewView($1)"]
pub fn respond_with_new_view_with_array_buffer_view(&self, view: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `respondWithNewView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respondWithNewView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
impl ReadableStreamByobRequest {
    #[js::import = "$0.respondWithNewView($1)"]
pub fn respond_with_new_view_with_u8_array(&self, view: &mut [u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `respondWithNewView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest/respondWithNewView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamByobRequest`*"]
impl ReadableStreamByobRequest {
    #[js::import = "$0.respondWithNewView($1)"]
pub fn respond_with_new_view_with_js_u8_array(&self, view: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
