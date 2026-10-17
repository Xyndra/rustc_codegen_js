#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpReceiver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpReceiver(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpReceiver`*"]
impl RtcRtpReceiver {
    #[js::import = "$0.track"]
pub fn track(&self) -> MediaStreamTrack { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `jitterBufferTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/jitterBufferTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpReceiver {
    #[js::import = "$0.jitterBufferTarget"]
pub fn jitter_buffer_target(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `jitterBufferTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/jitterBufferTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpReceiver {
    #[js::import = "$0.jitterBufferTarget = $1"]
pub fn set_jitter_buffer_target(&self, value: Option<f64>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpReceiver {
    #[js::import = "$0.transform"]
pub fn transform(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransform")]
    #[doc = "Setter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpReceiver {
    #[js::import = "$0.transform = $1"]
pub fn set_transform_opt_s_frame_transform(&self, value: Option<&SFrameTransform>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "RtcRtpScriptTransform")]
    #[doc = "Setter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcRtpScriptTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpReceiver {
    #[js::import = "$0.transform = $1"]
pub fn set_transform_opt_rtc_rtp_script_transform(&self, value: Option<&RtcRtpScriptTransform>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RtcRtpCapabilities")]
    #[doc = "The `getCapabilities()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getCapabilities_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`, `RtcRtpReceiver`*"]
impl RtcRtpReceiver {
    #[js::import = "RTCRtpReceiver.getCapabilities"]
pub fn get_capabilities(kind: &str) -> Option<RtcRtpCapabilities> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getContributingSources()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getContributingSources)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
impl RtcRtpReceiver {
    #[js::import = "$0.getContributingSources()"]
pub fn get_contributing_sources(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
impl RtcRtpReceiver {
    #[js::import = "$0.getStats()"]
pub fn get_stats(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `getSynchronizationSources()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpReceiver/getSynchronizationSources)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`*"]
impl RtcRtpReceiver {
    #[js::import = "$0.getSynchronizationSources()"]
pub fn get_synchronization_sources(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
