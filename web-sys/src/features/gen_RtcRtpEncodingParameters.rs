#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpEncodingParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpEncodingParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None"]
pub fn get_active(this: &RtcRtpEncodingParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None = $1"]
pub fn set_active(this: &RtcRtpEncodingParameters, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcDegradationPreference")]
    #[doc = "Get the `degradationPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDegradationPreference`, `RtcRtpEncodingParameters`*"]
#[js::import = "None"]
pub fn get_degradation_preference(this: &RtcRtpEncodingParameters,) -> Option<RtcDegradationPreference> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcDegradationPreference")]
    #[doc = "Change the `degradationPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDegradationPreference`, `RtcRtpEncodingParameters`*"]
#[js::import = "None = $1"]
pub fn set_degradation_preference(this: &RtcRtpEncodingParameters, val: RtcDegradationPreference,) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcFecParameters")]
    #[doc = "Get the `fec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`, `RtcRtpEncodingParameters`*"]
#[js::import = "None"]
pub fn get_fec(this: &RtcRtpEncodingParameters) -> Option<RtcFecParameters> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcFecParameters")]
    #[doc = "Change the `fec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`, `RtcRtpEncodingParameters`*"]
#[js::import = "None = $1"]
pub fn set_fec(this: &RtcRtpEncodingParameters, val: &RtcFecParameters) { panic!("web-sys intrinsic") }
    #[doc = "Get the `maxBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None"]
pub fn get_max_bitrate(this: &RtcRtpEncodingParameters) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `maxBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None = $1"]
pub fn set_max_bitrate(this: &RtcRtpEncodingParameters, val: u32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcPriorityType")]
    #[doc = "Get the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPriorityType`, `RtcRtpEncodingParameters`*"]
#[js::import = "None"]
pub fn get_priority(this: &RtcRtpEncodingParameters) -> Option<RtcPriorityType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcPriorityType")]
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPriorityType`, `RtcRtpEncodingParameters`*"]
#[js::import = "None = $1"]
pub fn set_priority(this: &RtcRtpEncodingParameters, val: RtcPriorityType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `rid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None"]
pub fn get_rid(this: &RtcRtpEncodingParameters) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None = $1"]
pub fn set_rid(this: &RtcRtpEncodingParameters, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtxParameters")]
    #[doc = "Get the `rtx` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`, `RtcRtxParameters`*"]
#[js::import = "None"]
pub fn get_rtx(this: &RtcRtpEncodingParameters) -> Option<RtcRtxParameters> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtxParameters")]
    #[doc = "Change the `rtx` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`, `RtcRtxParameters`*"]
#[js::import = "None = $1"]
pub fn set_rtx(this: &RtcRtpEncodingParameters, val: &RtcRtxParameters) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_scalability_mode(this: &RtcRtpEncodingParameters) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `scalabilityMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_scalability_mode(this: &RtcRtpEncodingParameters, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `scaleResolutionDownBy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None"]
pub fn get_scale_resolution_down_by(this: &RtcRtpEncodingParameters) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `scaleResolutionDownBy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None = $1"]
pub fn set_scale_resolution_down_by(this: &RtcRtpEncodingParameters, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None"]
pub fn get_ssrc(this: &RtcRtpEncodingParameters) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None = $1"]
pub fn set_ssrc(this: &RtcRtpEncodingParameters, val: u32) { panic!("web-sys intrinsic") }
impl RtcRtpEncodingParameters {
    #[doc = "Construct a new `RtcRtpEncodingParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpEncodingParameters`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
#[js::import = "None"]
pub fn active(&mut self, val: bool) -> &mut Self { self.set_active(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcDegradationPreference")]
    #[deprecated = "Use `set_degradation_preference()` instead."]
#[js::import = "None"]
pub fn degradation_preference(&mut self, val: RtcDegradationPreference) -> &mut Self { self.set_degradation_preference(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcFecParameters")]
    #[deprecated = "Use `set_fec()` instead."]
#[js::import = "None"]
pub fn fec(&mut self, val: &RtcFecParameters) -> &mut Self { self.set_fec(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_max_bitrate()` instead."]
#[js::import = "None"]
pub fn max_bitrate(&mut self, val: u32) -> &mut Self { self.set_max_bitrate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcPriorityType")]
    #[deprecated = "Use `set_priority()` instead."]
#[js::import = "None"]
pub fn priority(&mut self, val: RtcPriorityType) -> &mut Self { self.set_priority(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_rid()` instead."]
#[js::import = "None"]
pub fn rid(&mut self, val: &str) -> &mut Self { self.set_rid(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcRtxParameters")]
    #[deprecated = "Use `set_rtx()` instead."]
#[js::import = "None"]
pub fn rtx(&mut self, val: &RtcRtxParameters) -> &mut Self { self.set_rtx(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_scalability_mode()` instead."]
#[js::import = "None"]
pub fn scalability_mode(&mut self, val: &str) -> &mut Self { self.set_scalability_mode(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_scale_resolution_down_by()` instead."]
#[js::import = "None"]
pub fn scale_resolution_down_by(&mut self, val: f32) -> &mut Self { self.set_scale_resolution_down_by(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ssrc()` instead."]
#[js::import = "None"]
pub fn ssrc(&mut self, val: u32) -> &mut Self { self.set_ssrc(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcRtpEncodingParameters {
    fn default() -> Self {
        Self::new()
    }
}
