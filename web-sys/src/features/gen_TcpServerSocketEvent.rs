#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpServerSocketEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TcpServerSocketEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "TcpSocket")]
    #[doc = "Getter for the `socket` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/socket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEvent`, `TcpSocket`*"]
impl TcpServerSocketEvent {
    #[js::import = "$0.socket"]
pub fn socket(&self) -> TcpSocket { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TcpServerSocketEvent(..)` constructor, creating a new instance of `TcpServerSocketEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/TCPServerSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEvent`*"]
impl TcpServerSocketEvent {
    #[js::import = "new TCPServerSocketEvent($0)"]
pub fn new(type_: &str) -> Result<TcpServerSocketEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TcpServerSocketEventInit")]
    #[doc = "The `new TcpServerSocketEvent(..)` constructor, creating a new instance of `TcpServerSocketEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocketEvent/TCPServerSocketEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocketEvent`, `TcpServerSocketEventInit`*"]
impl TcpServerSocketEvent {
    #[js::import = "new TCPServerSocketEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &TcpServerSocketEventInit,) -> Result<TcpServerSocketEvent, JsValue> { panic!("web-sys intrinsic") }
}
