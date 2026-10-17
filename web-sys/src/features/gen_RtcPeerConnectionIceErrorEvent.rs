#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcPeerConnectionIceErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcPeerConnectionIceErrorEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `address` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/address)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
impl RtcPeerConnectionIceErrorEvent {
    #[js::import = "$0.address"]
pub fn address(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
impl RtcPeerConnectionIceErrorEvent {
    #[js::import = "$0.port"]
pub fn port(&self) -> Option<u16> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
impl RtcPeerConnectionIceErrorEvent {
    #[js::import = "$0.url"]
pub fn url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/errorCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
impl RtcPeerConnectionIceErrorEvent {
    #[js::import = "$0.errorCode"]
pub fn error_code(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `errorText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnectionIceErrorEvent/errorText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceErrorEvent`*"]
impl RtcPeerConnectionIceErrorEvent {
    #[js::import = "$0.errorText"]
pub fn error_text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
