#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TcpServerSocket` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TcpServerSocket(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/localPort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
impl TcpServerSocket {
    #[js::import = "$0.localPort"]
pub fn local_port(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
impl TcpServerSocket {
    #[js::import = "$0.onconnect"]
pub fn onconnect(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
impl TcpServerSocket {
    #[js::import = "$0.onconnect = $1"]
pub fn set_onconnect(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
impl TcpServerSocket {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
impl TcpServerSocket {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TcpServerSocket(..)` constructor, creating a new instance of `TcpServerSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
impl TcpServerSocket {
    #[js::import = "new TCPServerSocket($0)"]
pub fn new(port: u16) -> Result<TcpServerSocket, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServerSocketOptions")]
    #[doc = "The `new TcpServerSocket(..)` constructor, creating a new instance of `TcpServerSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpServerSocket`*"]
impl TcpServerSocket {
    #[js::import = "new TCPServerSocket($0, $1)"]
pub fn new_with_options(port: u16, options: &ServerSocketOptions,) -> Result<TcpServerSocket, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServerSocketOptions")]
    #[doc = "The `new TcpServerSocket(..)` constructor, creating a new instance of `TcpServerSocket`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/TCPServerSocket)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpServerSocket`*"]
impl TcpServerSocket {
    #[js::import = "new TCPServerSocket($0, $1, $2)"]
pub fn new_with_options_and_backlog(port: u16, options: &ServerSocketOptions, backlog: u16,) -> Result<TcpServerSocket, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TCPServerSocket/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TcpServerSocket`*"]
impl TcpServerSocket {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
