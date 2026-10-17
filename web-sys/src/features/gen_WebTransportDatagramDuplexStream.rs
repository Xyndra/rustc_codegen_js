#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportDatagramDuplexStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebTransportDatagramDuplexStream(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.readable"]
pub fn readable(&self) -> ReadableStream { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.writable"]
pub fn writable(&self) -> WritableStream { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxDatagramSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/maxDatagramSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.maxDatagramSize"]
pub fn max_datagram_size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `incomingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingMaxAge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.incomingMaxAge"]
pub fn incoming_max_age(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `incomingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingMaxAge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.incomingMaxAge = $1"]
pub fn set_incoming_max_age(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `outgoingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingMaxAge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.outgoingMaxAge"]
pub fn outgoing_max_age(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `outgoingMaxAge` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingMaxAge)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.outgoingMaxAge = $1"]
pub fn set_outgoing_max_age(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `incomingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingHighWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.incomingHighWaterMark"]
pub fn incoming_high_water_mark(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `incomingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/incomingHighWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.incomingHighWaterMark = $1"]
pub fn set_incoming_high_water_mark(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `outgoingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingHighWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.outgoingHighWaterMark"]
pub fn outgoing_high_water_mark(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `outgoingHighWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportDatagramDuplexStream/outgoingHighWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportDatagramDuplexStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportDatagramDuplexStream {
    #[js::import = "$0.outgoingHighWaterMark = $1"]
pub fn set_outgoing_high_water_mark(&self, value: f64) { panic!("web-sys intrinsic") }
}
