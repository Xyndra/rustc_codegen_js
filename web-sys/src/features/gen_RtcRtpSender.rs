#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpSender` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpSender(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*"]
impl RtcRtpSender {
    #[js::import = "$0.track"]
pub fn track(&self) -> Option<MediaStreamTrack> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcdtmfSender")]
    #[doc = "Getter for the `dtmf` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/dtmf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`, `RtcdtmfSender`*"]
impl RtcRtpSender {
    #[js::import = "$0.dtmf"]
pub fn dtmf(&self) -> Option<RtcdtmfSender> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpSender {
    #[js::import = "$0.transform"]
pub fn transform(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransform")]
    #[doc = "Setter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpSender {
    #[js::import = "$0.transform = $1"]
pub fn set_transform_opt_s_frame_transform(&self, value: Option<&SFrameTransform>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcRtpScriptTransform")]
    #[doc = "Setter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`, `RtcRtpSender`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpSender {
    #[js::import = "$0.transform = $1"]
pub fn set_transform_opt_rtc_rtp_script_transform(&self, value: Option<&RtcRtpScriptTransform>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `generateKeyFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/generateKeyFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpSender {
    #[js::import = "$0.generateKeyFrame()"]
pub fn generate_key_frame(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `generateKeyFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/generateKeyFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpSender {
    #[js::import = "$0.generateKeyFrame($1)"]
pub fn generate_key_frame_with_rids(&self, rids: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpCapabilities")]
    #[doc = "The `getCapabilities()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getCapabilities_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`, `RtcRtpSender`*"]
impl RtcRtpSender {
    #[js::import = "RTCRtpSender.getCapabilities"]
pub fn get_capabilities(kind: &str) -> Option<RtcRtpCapabilities> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpParameters")]
    #[doc = "The `getParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*"]
impl RtcRtpSender {
    #[js::import = "$0.getParameters()"]
pub fn get_parameters(&self) -> RtcRtpParameters { panic!("web-sys intrinsic") }
}
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
impl RtcRtpSender {
    #[js::import = "$0.getStats()"]
pub fn get_stats(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "The `replaceTrack()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/replaceTrack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpSender`*"]
impl RtcRtpSender {
    #[js::import = "$0.replaceTrack($1)"]
pub fn replace_track(&self, with_track: Option<&MediaStreamTrack>) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `setParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSender`*"]
impl RtcRtpSender {
    #[js::import = "$0.setParameters()"]
pub fn set_parameters(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpParameters")]
    #[doc = "The `setParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpSender/setParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`, `RtcRtpSender`*"]
impl RtcRtpSender {
    #[js::import = "$0.setParameters($1)"]
pub fn set_parameters_with_parameters(&self, parameters: &RtcRtpParameters) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
