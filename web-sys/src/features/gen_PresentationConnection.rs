#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PresentationConnection(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.url"]
pub fn url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PresentationConnectionState")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionState`*"]
impl PresentationConnection {
    #[js::import = "$0.state"]
pub fn state(&self) -> PresentationConnectionState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.onconnect"]
pub fn onconnect(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.onconnect = $1"]
pub fn set_onconnect(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.onclose"]
pub fn onclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.onclose = $1"]
pub fn set_onclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onterminate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.onterminate"]
pub fn onterminate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onterminate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onterminate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.onterminate = $1"]
pub fn set_onterminate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PresentationConnectionBinaryType")]
    #[doc = "Getter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*"]
impl PresentationConnection {
    #[js::import = "$0.binaryType"]
pub fn binary_type(&self) -> PresentationConnectionBinaryType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PresentationConnectionBinaryType")]
    #[doc = "Setter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionBinaryType`*"]
impl PresentationConnection {
    #[js::import = "$0.binaryType = $1"]
pub fn set_binary_type(&self, value: PresentationConnectionBinaryType) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.close()"]
pub fn close(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.send($1)"]
pub fn send_with_str(&self, data: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.send($1)"]
pub fn send_with_blob(&self, data: &Blob) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.send($1)"]
pub fn send_with_array_buffer(&self, data: &::js_sys::ArrayBuffer) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.send($1)"]
pub fn send_with_array_buffer_view(&self, data: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.send($1)"]
pub fn send_with_u8_array(&self, data: &[u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.send($1)"]
pub fn send_with_js_u8_array(&self, data: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `terminate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnection/terminate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`*"]
impl PresentationConnection {
    #[js::import = "$0.terminate()"]
pub fn terminate(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
