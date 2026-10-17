#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushMessageData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushMessageData`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushMessageData(pub ::wasm_bindgen::JsValue);
    #[doc = "The `arrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/arrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushMessageData`*"]
impl PushMessageData {
    #[js::import = "$0.arrayBuffer()"]
pub fn array_buffer(&self) -> Result<::js_sys::ArrayBuffer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `blob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `PushMessageData`*"]
impl PushMessageData {
    #[js::import = "$0.blob()"]
pub fn blob(&self) -> Result<Blob, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `json()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/json)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushMessageData`*"]
impl PushMessageData {
    #[js::import = "$0.json()"]
pub fn json(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `text()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushMessageData/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushMessageData`*"]
impl PushMessageData {
    #[js::import = "$0.text()"]
pub fn text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
