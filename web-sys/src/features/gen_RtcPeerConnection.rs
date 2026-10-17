#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcPeerConnection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcPeerConnection(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `localDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/localDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
impl RtcPeerConnection {
    #[js::import = "$0.localDescription"]
pub fn local_description(&self) -> Option<RtcSessionDescription> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `currentLocalDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentLocalDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
impl RtcPeerConnection {
    #[js::import = "$0.currentLocalDescription"]
pub fn current_local_description(&self) -> Option<RtcSessionDescription> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `pendingLocalDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingLocalDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
impl RtcPeerConnection {
    #[js::import = "$0.pendingLocalDescription"]
pub fn pending_local_description(&self) -> Option<RtcSessionDescription> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `remoteDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/remoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
impl RtcPeerConnection {
    #[js::import = "$0.remoteDescription"]
pub fn remote_description(&self) -> Option<RtcSessionDescription> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `currentRemoteDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/currentRemoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
impl RtcPeerConnection {
    #[js::import = "$0.currentRemoteDescription"]
pub fn current_remote_description(&self) -> Option<RtcSessionDescription> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescription")]
    #[doc = "Getter for the `pendingRemoteDescription` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/pendingRemoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescription`*"]
impl RtcPeerConnection {
    #[js::import = "$0.pendingRemoteDescription"]
pub fn pending_remote_description(&self) -> Option<RtcSessionDescription> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSignalingState")]
    #[doc = "Getter for the `signalingState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/signalingState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSignalingState`*"]
impl RtcPeerConnection {
    #[js::import = "$0.signalingState"]
pub fn signaling_state(&self) -> RtcSignalingState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `canTrickleIceCandidates` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/canTrickleIceCandidates)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.canTrickleIceCandidates"]
pub fn can_trickle_ice_candidates(&self) -> Option<bool> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcIceGatheringState")]
    #[doc = "Getter for the `iceGatheringState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceGatheringState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceGatheringState`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.iceGatheringState"]
pub fn ice_gathering_state(&self) -> RtcIceGatheringState { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcIceConnectionState")]
    #[doc = "Getter for the `iceConnectionState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/iceConnectionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceConnectionState`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.iceConnectionState"]
pub fn ice_connection_state(&self) -> RtcIceConnectionState { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcPeerConnectionState")]
    #[doc = "Getter for the `connectionState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/connectionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcPeerConnectionState`*"]
impl RtcPeerConnection {
    #[js::import = "$0.connectionState"]
pub fn connection_state(&self) -> RtcPeerConnectionState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/peerIdentity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.peerIdentity"]
pub fn peer_identity(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `idpLoginUrl` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/idpLoginUrl)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.idpLoginUrl"]
pub fn idp_login_url(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onnegotiationneeded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onnegotiationneeded"]
pub fn onnegotiationneeded(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onnegotiationneeded` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onnegotiationneeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onnegotiationneeded = $1"]
pub fn set_onnegotiationneeded(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onicecandidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onicecandidate"]
pub fn onicecandidate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onicecandidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicecandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onicecandidate = $1"]
pub fn set_onicecandidate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsignalingstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onsignalingstatechange"]
pub fn onsignalingstatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsignalingstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onsignalingstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onsignalingstatechange = $1"]
pub fn set_onsignalingstatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onaddstream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddstream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onaddstream"]
pub fn onaddstream(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onaddstream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddstream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onaddstream = $1"]
pub fn set_onaddstream(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onaddtrack"]
pub fn onaddtrack(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onaddtrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onaddtrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onaddtrack = $1"]
pub fn set_onaddtrack(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.ontrack"]
pub fn ontrack(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontrack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ontrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.ontrack = $1"]
pub fn set_ontrack(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onremovestream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onremovestream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onremovestream"]
pub fn onremovestream(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onremovestream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onremovestream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onremovestream = $1"]
pub fn set_onremovestream(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oniceconnectionstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.oniceconnectionstatechange"]
pub fn oniceconnectionstatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oniceconnectionstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/oniceconnectionstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.oniceconnectionstatechange = $1"]
pub fn set_oniceconnectionstatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onicegatheringstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onicegatheringstatechange"]
pub fn onicegatheringstatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onicegatheringstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onicegatheringstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onicegatheringstatechange = $1"]
pub fn set_onicegatheringstatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onconnectionstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onconnectionstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onconnectionstatechange"]
pub fn onconnectionstatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onconnectionstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/onconnectionstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.onconnectionstatechange = $1"]
pub fn set_onconnectionstatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondatachannel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.ondatachannel"]
pub fn ondatachannel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondatachannel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/ondatachannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.ondatachannel = $1"]
pub fn set_ondatachannel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new RtcPeerConnection(..)` constructor, creating a new instance of `RtcPeerConnection`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "new RTCPeerConnection()"]
pub fn new() -> Result<RtcPeerConnection, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcConfiguration")]
    #[doc = "The `new RtcPeerConnection(..)` constructor, creating a new instance of `RtcPeerConnection`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "new RTCPeerConnection($0)"]
pub fn new_with_configuration(configuration: &RtcConfiguration,) -> Result<RtcPeerConnection, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcConfiguration")]
    #[doc = "The `new RtcPeerConnection(..)` constructor, creating a new instance of `RtcPeerConnection`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/RTCPeerConnection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "new RTCPeerConnection($0, $1)"]
pub fn new_with_configuration_and_constraints(configuration: &RtcConfiguration, constraints: Option<&::js_sys::Object>,) -> Result<RtcPeerConnection, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcIceCandidateInit")]
    #[doc = "The `addIceCandidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addIceCandidate($1)"]
pub fn add_ice_candidate_with_opt_rtc_ice_candidate_init(&self, candidate: Option<&RtcIceCandidateInit>) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcIceCandidate")]
    #[doc = "The `addIceCandidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addIceCandidate($1)"]
pub fn add_ice_candidate_with_opt_rtc_ice_candidate(&self, candidate: Option<&RtcIceCandidate>) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcIceCandidate")]
    #[doc = "The `addIceCandidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addIceCandidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addIceCandidate($1, $2, $3)"]
pub fn add_ice_candidate_with_rtc_ice_candidate_and_success_callback_and_failure_callback(&self, candidate: &RtcIceCandidate, success_callback: &::js_sys::Function, failure_callback: &::js_sys::Function) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStream")]
    #[doc = "The `addStream()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addStream($1)"]
pub fn add_stream(&self, stream: &MediaStream) { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTrack($1, $2, $3)"]
pub fn add_track(&self, track: &MediaStreamTrack, stream: &MediaStream, more_streams: &::js_sys::Array) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTrack($1, $2)"]
pub fn add_track_0(&self, track: &MediaStreamTrack, stream: &MediaStream) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTrack($1, $2, $3)"]
pub fn add_track_1(&self, track: &MediaStreamTrack, stream: &MediaStream, more_streams_1: &MediaStream) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTrack($1, $2, $3, $4)"]
pub fn add_track_2(&self, track: &MediaStreamTrack, stream: &MediaStream, more_streams_1: &MediaStream, more_streams_2: &MediaStream) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTrack($1, $2, $3, $4, $5)"]
pub fn add_track_3(&self, track: &MediaStreamTrack, stream: &MediaStream, more_streams_1: &MediaStream, more_streams_2: &MediaStream, more_streams_3: &MediaStream) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTrack($1, $2, $3, $4, $5, $6)"]
pub fn add_track_4(&self, track: &MediaStreamTrack, stream: &MediaStream, more_streams_1: &MediaStream, more_streams_2: &MediaStream, more_streams_3: &MediaStream, more_streams_4: &MediaStream) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTrack($1, $2, $3, $4, $5, $6, $7)"]
pub fn add_track_5(&self, track: &MediaStreamTrack, stream: &MediaStream, more_streams_1: &MediaStream, more_streams_2: &MediaStream, more_streams_3: &MediaStream, more_streams_4: &MediaStream, more_streams_5: &MediaStream) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTrack($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn add_track_6(&self, track: &MediaStreamTrack, stream: &MediaStream, more_streams_1: &MediaStream, more_streams_2: &MediaStream, more_streams_3: &MediaStream, more_streams_4: &MediaStream, more_streams_5: &MediaStream, more_streams_6: &MediaStream) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStream",
        feature = "MediaStreamTrack",
        feature = "RtcRtpSender",
    ))]
    #[doc = "The `addTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTrack($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn add_track_7(&self, track: &MediaStreamTrack, stream: &MediaStream, more_streams_1: &MediaStream, more_streams_2: &MediaStream, more_streams_3: &MediaStream, more_streams_4: &MediaStream, more_streams_5: &MediaStream, more_streams_6: &MediaStream, more_streams_7: &MediaStream) -> RtcRtpSender { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "MediaStreamTrack", feature = "RtcRtpTransceiver",))]
    #[doc = "The `addTransceiver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpTransceiver`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTransceiver($1)"]
pub fn add_transceiver_with_media_stream_track(&self, track_or_kind: &MediaStreamTrack) -> RtcRtpTransceiver { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "The `addTransceiver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpTransceiver`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTransceiver($1)"]
pub fn add_transceiver_with_str(&self, track_or_kind: &str) -> RtcRtpTransceiver { panic!("web-sys intrinsic") }
}
    #[cfg(all(
        feature = "MediaStreamTrack",
        feature = "RtcRtpTransceiver",
        feature = "RtcRtpTransceiverInit",
    ))]
    #[doc = "The `addTransceiver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`, `RtcRtpTransceiver`, `RtcRtpTransceiverInit`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTransceiver($1, $2)"]
pub fn add_transceiver_with_media_stream_track_and_init(&self, track_or_kind: &MediaStreamTrack, init: &RtcRtpTransceiverInit) -> RtcRtpTransceiver { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "RtcRtpTransceiver", feature = "RtcRtpTransceiverInit",))]
    #[doc = "The `addTransceiver()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/addTransceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpTransceiver`, `RtcRtpTransceiverInit`*"]
impl RtcPeerConnection {
    #[js::import = "$0.addTransceiver($1, $2)"]
pub fn add_transceiver_with_str_and_init(&self, track_or_kind: &str, init: &RtcRtpTransceiverInit) -> RtcRtpTransceiver { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `createAnswer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.createAnswer()"]
pub fn create_answer(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcAnswerOptions")]
    #[doc = "The `createAnswer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcAnswerOptions`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.createAnswer($1)"]
pub fn create_answer_with_rtc_answer_options(&self, options: &RtcAnswerOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `createAnswer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createAnswer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.createAnswer($1, $2)"]
pub fn create_answer_with_success_callback_and_failure_callback(&self, success_callback: &::js_sys::Function, failure_callback: &::js_sys::Function) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcDataChannel")]
    #[doc = "The `createDataChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.createDataChannel($1)"]
pub fn create_data_channel(&self, label: &str) -> RtcDataChannel { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "RtcDataChannel", feature = "RtcDataChannelInit",))]
    #[doc = "The `createDataChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createDataChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannel`, `RtcDataChannelInit`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.createDataChannel($1, $2)"]
pub fn create_data_channel_with_data_channel_dict(&self, label: &str, data_channel_dict: &RtcDataChannelInit) -> RtcDataChannel { panic!("web-sys intrinsic") }
}
    #[doc = "The `createOffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.createOffer()"]
pub fn create_offer(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcOfferOptions")]
    #[doc = "The `createOffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.createOffer($1)"]
pub fn create_offer_with_rtc_offer_options(&self, options: &RtcOfferOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `createOffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.createOffer($1, $2)"]
pub fn create_offer_with_callback_and_failure_callback(&self, success_callback: &::js_sys::Function, failure_callback: &::js_sys::Function) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcOfferOptions")]
    #[doc = "The `createOffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/createOffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOfferOptions`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.createOffer($1, $2, $3)"]
pub fn create_offer_with_callback_and_failure_callback_and_options(&self, success_callback: &::js_sys::Function, failure_callback: &::js_sys::Function, options: &RtcOfferOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `generateCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "RTCPeerConnection.generateCertificate"]
pub fn generate_certificate_with_object(keygen_algorithm: &::js_sys::Object,) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `generateCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/generateCertificate_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "RTCPeerConnection.generateCertificate"]
pub fn generate_certificate_with_str(keygen_algorithm: &str,) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcConfiguration")]
    #[doc = "The `getConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getConfiguration()"]
pub fn get_configuration(&self) -> RtcConfiguration { panic!("web-sys intrinsic") }
}
    #[doc = "The `getIdentityAssertion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getIdentityAssertion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getIdentityAssertion()"]
pub fn get_identity_assertion(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `getLocalStreams()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getLocalStreams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getLocalStreams()"]
pub fn get_local_streams(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getReceivers()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getReceivers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getReceivers()"]
pub fn get_receivers(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRemoteStreams()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getRemoteStreams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getRemoteStreams()"]
pub fn get_remote_streams(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getSenders()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getSenders)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getSenders()"]
pub fn get_senders(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getStats()"]
pub fn get_stats(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getStats($1)"]
pub fn get_stats_with_selector(&self, selector: Option<&MediaStreamTrack>) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getStats($1, $2, $3)"]
pub fn get_stats_with_selector_and_success_callback_and_failure_callback(&self, selector: Option<&MediaStreamTrack>, success_callback: &::js_sys::Function, failure_callback: &::js_sys::Function) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `getTransceivers()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/getTransceivers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.getTransceivers()"]
pub fn get_transceivers(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpSender")]
    #[doc = "The `removeTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/removeTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcRtpSender`*"]
impl RtcPeerConnection {
    #[js::import = "$0.removeTrack($1)"]
pub fn remove_track(&self, sender: &RtcRtpSender) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.setConfiguration()"]
pub fn set_configuration(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcConfiguration")]
    #[doc = "The `setConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.setConfiguration($1)"]
pub fn set_configuration_with_configuration(&self, configuration: &RtcConfiguration) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setIdentityProvider()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.setIdentityProvider($1)"]
pub fn set_identity_provider(&self, provider: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcIdentityProviderOptions")]
    #[doc = "The `setIdentityProvider()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setIdentityProvider)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProviderOptions`, `RtcPeerConnection`*"]
impl RtcPeerConnection {
    #[js::import = "$0.setIdentityProvider($1, $2)"]
pub fn set_identity_provider_with_options(&self, provider: &str, options: &RtcIdentityProviderOptions) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[doc = "The `setLocalDescription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
impl RtcPeerConnection {
    #[js::import = "$0.setLocalDescription($1)"]
pub fn set_local_description(&self, description: &RtcSessionDescriptionInit) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[doc = "The `setLocalDescription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setLocalDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
impl RtcPeerConnection {
    #[js::import = "$0.setLocalDescription($1, $2, $3)"]
pub fn set_local_description_with_success_callback_and_failure_callback(&self, description: &RtcSessionDescriptionInit, success_callback: &::js_sys::Function, failure_callback: &::js_sys::Function) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[doc = "The `setRemoteDescription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
impl RtcPeerConnection {
    #[js::import = "$0.setRemoteDescription($1)"]
pub fn set_remote_description(&self, description: &RtcSessionDescriptionInit) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcSessionDescriptionInit")]
    #[doc = "The `setRemoteDescription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCPeerConnection/setRemoteDescription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnection`, `RtcSessionDescriptionInit`*"]
impl RtcPeerConnection {
    #[js::import = "$0.setRemoteDescription($1, $2, $3)"]
pub fn set_remote_description_with_success_callback_and_failure_callback(&self, description: &RtcSessionDescriptionInit, success_callback: &::js_sys::Function, failure_callback: &::js_sys::Function) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
