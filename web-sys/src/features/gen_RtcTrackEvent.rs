#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTrackEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcTrackEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Getter for the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/receiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEvent`*"]
impl RtcTrackEvent {
    #[js::import = "$0.receiver"]
pub fn receiver(&self) -> RtcRtpReceiver { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEvent`*"]
impl RtcTrackEvent {
    #[js::import = "$0.track"]
pub fn track(&self) -> MediaStreamTrack { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `streams` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/streams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEvent`*"]
impl RtcTrackEvent {
    #[js::import = "$0.streams"]
pub fn streams(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "Getter for the `transceiver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/transceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEvent`*"]
impl RtcTrackEvent {
    #[js::import = "$0.transceiver"]
pub fn transceiver(&self) -> RtcRtpTransceiver { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcTrackEventInit")]
    #[doc = "The `new RtcTrackEvent(..)` constructor, creating a new instance of `RtcTrackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCTrackEvent/RTCTrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEvent`, `RtcTrackEventInit`*"]
impl RtcTrackEvent {
    #[js::import = "new RTCTrackEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &RtcTrackEventInit) -> Result<RtcTrackEvent, JsValue> { panic!("web-sys intrinsic") }
}
