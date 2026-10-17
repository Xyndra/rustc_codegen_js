#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportBidirectionalStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportBidirectionalStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebTransportBidirectionalStream(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportReceiveStream")]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportBidirectionalStream`, `WebTransportReceiveStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportBidirectionalStream {
    #[js::import = "$0.readable"]
pub fn readable(&self) -> WebTransportReceiveStream { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportSendStream")]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportBidirectionalStream/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportBidirectionalStream`, `WebTransportSendStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportBidirectionalStream {
    #[js::import = "$0.writable"]
pub fn writable(&self) -> WebTransportSendStream { panic!("web-sys intrinsic") }
}
