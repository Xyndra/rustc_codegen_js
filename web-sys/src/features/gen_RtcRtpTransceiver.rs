#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpTransceiver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpTransceiver(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `mid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/mid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.mid"]
pub fn mid(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpSender")]
    #[doc = "Getter for the `sender` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/sender)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`, `RtcRtpTransceiver`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.sender"]
pub fn sender(&self) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Getter for the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/receiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcRtpTransceiver`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.receiver"]
pub fn receiver(&self) -> RtcRtpReceiver { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `stopped` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/stopped)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.stopped"]
pub fn stopped(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Getter for the `direction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.direction"]
pub fn direction(&self) -> RtcRtpTransceiverDirection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Setter for the `direction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/direction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.direction = $1"]
pub fn set_direction(&self, value: RtcRtpTransceiverDirection) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Getter for the `currentDirection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/currentDirection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcRtpTransceiverDirection`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.currentDirection"]
pub fn current_direction(&self) -> Option<RtcRtpTransceiverDirection> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRemoteTrackId()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/getRemoteTrackId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.getRemoteTrackId()"]
pub fn get_remote_track_id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCodecPreferences()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/setCodecPreferences)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.setCodecPreferences($1)"]
pub fn set_codec_preferences(&self, codecs: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpTransceiver/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`*"]
impl RtcRtpTransceiver {
    #[js::import = "$0.stop()"]
pub fn stop(&self) { panic!("web-sys intrinsic") }
}
