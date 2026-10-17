#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocket` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TcpSocket(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `host` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/host)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.host"]
pub fn host(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.port"]
pub fn port(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ssl)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.ssl"]
pub fn ssl(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bufferedAmount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/bufferedAmount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.bufferedAmount"]
pub fn buffered_amount(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TcpReadyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpReadyState`, `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> TcpReadyState { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Getter for the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/binaryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`, `TcpSocketBinaryType`*"]
impl TcpSocket {
    #[js::import = "$0.binaryType"]
pub fn binary_type(&self) -> TcpSocketBinaryType { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.onopen"]
pub fn onopen(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.onopen = $1"]
pub fn set_onopen(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondrain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.ondrain"]
pub fn ondrain(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondrain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondrain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.ondrain = $1"]
pub fn set_ondrain(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.ondata"]
pub fn ondata(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/ondata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.ondata = $1"]
pub fn set_ondata(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.onclose"]
pub fn onclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.onclose = $1"]
pub fn set_onclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TcpSocket(..)` constructor, creating a new instance of `TcpSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "new TCPSocket($0, $1)"]
pub fn new(host: &str, port: u16) -> Result<TcpSocket, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SocketOptions")]
    #[doc = "The `new TcpSocket(..)` constructor, creating a new instance of `TcpSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/TCPSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "new TCPSocket($0, $1, $2)"]
pub fn new_with_options(host: &str, port: u16, options: &SocketOptions,) -> Result<TcpSocket, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `resume()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/resume)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.resume()"]
pub fn resume(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.send($1)"]
pub fn send_with_str(&self, data: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.send($1)"]
pub fn send_with_array_buffer(&self, data: &::js_sys::ArrayBuffer) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.send($1, $2)"]
pub fn send_with_array_buffer_and_byte_offset(&self, data: &::js_sys::ArrayBuffer, byte_offset: u32) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.send($1, $2, $3)"]
pub fn send_with_array_buffer_and_byte_offset_and_byte_length(&self, data: &::js_sys::ArrayBuffer, byte_offset: u32, byte_length: u32) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `suspend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/suspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.suspend()"]
pub fn suspend(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `upgradeToSecure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocket/upgradeToSecure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocket`*"]
impl TcpSocket {
    #[js::import = "$0.upgradeToSecure()"]
pub fn upgrade_to_secure(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
