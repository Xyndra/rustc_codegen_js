#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpSocketErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TcpSocketErrorEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
impl TcpSocketErrorEvent {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
impl TcpSocketErrorEvent {
    #[js::import = "$0.message"]
pub fn message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TcpSocketErrorEvent(..)` constructor, creating a new instance of `TcpSocketErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`*"]
impl TcpSocketErrorEvent {
    #[js::import = "new TCPSocketErrorEvent($0)"]
pub fn new(type_: &str) -> Result<TcpSocketErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TcpSocketErrorEventInit")]
    #[doc = "The `new TcpSocketErrorEvent(..)` constructor, creating a new instance of `TcpSocketErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPSocketErrorEvent/TCPSocketErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpSocketErrorEvent`, `TcpSocketErrorEventInit`*"]
impl TcpSocketErrorEvent {
    #[js::import = "new TCPSocketErrorEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &TcpSocketErrorEventInit,) -> Result<TcpSocketErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}
