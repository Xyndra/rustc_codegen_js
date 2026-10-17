#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocketEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TcpSocketEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
impl TcpSocketEvent {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TcpSocketEvent(..)` constructor, creating a new instance of `TcpSocketEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketEvent`*"]
impl TcpSocketEvent {
    #[js::import = "new TCPSocketEvent($0)"]
pub fn new(type_: &str) -> Result<TcpSocketEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TcpSocketEventInit")]
    #[doc = "The `new TcpSocketEvent(..)` constructor, creating a new instance of `TcpSocketEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketEvent/TCPSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketEvent`, `TcpSocketEventInit`*"]
impl TcpSocketEvent {
    #[js::import = "new TCPSocketEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &TcpSocketEventInit,) -> Result<TcpSocketEvent, JsValue> { panic!("web-sys intrinsic") }
}
