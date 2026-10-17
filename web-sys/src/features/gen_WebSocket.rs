#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebSocket` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebSocket(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.url"]
pub fn url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bufferedAmount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/bufferedAmount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.bufferedAmount"]
pub fn buffered_amount(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.onopen"]
pub fn onopen(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.onopen = $1"]
pub fn set_onopen(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.onclose"]
pub fn onclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.onclose = $1"]
pub fn set_onclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `extensions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/extensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.extensions"]
pub fn extensions(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/protocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.protocol"]
pub fn protocol(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BinaryType")]
    #[doc = "Getter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.binaryType"]
pub fn binary_type(&self) -> BinaryType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BinaryType")]
    #[doc = "Setter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BinaryType`, `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.binaryType = $1"]
pub fn set_binary_type(&self, value: BinaryType) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "new WebSocket($0)"]
pub fn new(url: &str) -> Result<WebSocket, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "new WebSocket($0, $1)"]
pub fn new_with_str(url: &str, protocols: &str) -> Result<WebSocket, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new WebSocket(..)` constructor, creating a new instance of `WebSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/WebSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "new WebSocket($0, $1)"]
pub fn new_with_str_sequence(url: &str, protocols: &::wasm_bindgen::JsValue,) -> Result<WebSocket, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.close()"]
pub fn close(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.close($1)"]
pub fn close_with_code(&self, code: u16) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.close($1, $2)"]
pub fn close_with_code_and_reason(&self, code: u16, reason: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.send($1)"]
pub fn send_with_str(&self, data: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.send($1)"]
pub fn send_with_blob(&self, data: &Blob) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.send($1)"]
pub fn send_with_array_buffer(&self, data: &::js_sys::ArrayBuffer) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.send($1)"]
pub fn send_with_array_buffer_view(&self, data: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.send($1)"]
pub fn send_with_u8_array(&self, data: &[u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
impl WebSocket {
    #[js::import = "$0.send($1)"]
pub fn send_with_js_u8_array(&self, data: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl WebSocket {
    #[doc = "The `WebSocket.CONNECTING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub const CONNECTING: u16 = 0i64 as u16;
    #[doc = "The `WebSocket.OPEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub const OPEN: u16 = 1u64 as u16;
    #[doc = "The `WebSocket.CLOSING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub const CLOSING: u16 = 2u64 as u16;
    #[doc = "The `WebSocket.CLOSED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocket`*"]
    pub const CLOSED: u16 = 3u64 as u16;
}
