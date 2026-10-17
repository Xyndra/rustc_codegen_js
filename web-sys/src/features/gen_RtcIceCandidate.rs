#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidate` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIceCandidate(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
impl RtcIceCandidate {
    #[js::import = "$0.candidate"]
pub fn candidate(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/candidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
impl RtcIceCandidate {
    #[js::import = "$0.candidate = $1"]
pub fn set_candidate(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
impl RtcIceCandidate {
    #[js::import = "$0.sdpMid"]
pub fn sdp_mid(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
impl RtcIceCandidate {
    #[js::import = "$0.sdpMid = $1"]
pub fn set_sdp_mid(&self, value: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
impl RtcIceCandidate {
    #[js::import = "$0.sdpMLineIndex"]
pub fn sdp_m_line_index(&self) -> Option<u16> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/sdpMLineIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
impl RtcIceCandidate {
    #[js::import = "$0.sdpMLineIndex = $1"]
pub fn set_sdp_m_line_index(&self, value: Option<u16>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcIceCandidateInit")]
    #[doc = "The `new RtcIceCandidate(..)` constructor, creating a new instance of `RtcIceCandidate`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/RTCIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcIceCandidateInit`*"]
impl RtcIceCandidate {
    #[js::import = "new RTCIceCandidate($0)"]
pub fn new(candidate_init_dict: &RtcIceCandidateInit) -> Result<RtcIceCandidate, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceCandidate/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`*"]
impl RtcIceCandidate {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
