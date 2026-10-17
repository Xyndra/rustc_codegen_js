#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Client` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Client(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
impl Client {
    #[js::import = "$0.url"]
pub fn url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FrameType")]
    #[doc = "Getter for the `frameType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/frameType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`, `FrameType`*"]
impl Client {
    #[js::import = "$0.frameType"]
pub fn frame_type(&self) -> FrameType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ClientType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`, `ClientType`*"]
impl Client {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ClientType { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
impl Client {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
impl Client {
    #[js::import = "$0.postMessage($1)"]
pub fn post_message(&self, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Client/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Client`*"]
impl Client {
    #[js::import = "$0.postMessage($1, $2)"]
pub fn post_message_with_transfer(&self, message: &::wasm_bindgen::JsValue, transfer: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
