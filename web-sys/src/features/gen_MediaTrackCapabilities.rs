#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackCapabilities` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaTrackCapabilities(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Get the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_aspect_ratio(this: &MediaTrackCapabilities) -> Option<DoubleRange> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Change the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_aspect_ratio(this: &MediaTrackCapabilities, val: &DoubleRange) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_auto_gain_control(this: &MediaTrackCapabilities) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_auto_gain_control(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `backgroundBlur` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_background_blur(this: &MediaTrackCapabilities) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `backgroundBlur` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_background_blur(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_channel_count(this: &MediaTrackCapabilities) -> Option<ULongRange> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &MediaTrackCapabilities, val: &ULongRange) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_device_id(this: &MediaTrackCapabilities) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_device_id(this: &MediaTrackCapabilities, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_echo_cancellation(this: &MediaTrackCapabilities) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_echo_cancellation(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_facing_mode(this: &MediaTrackCapabilities) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_facing_mode(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_frame_rate(this: &MediaTrackCapabilities) -> Option<DoubleRange> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_frame_rate(this: &MediaTrackCapabilities, val: &DoubleRange) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_group_id(this: &MediaTrackCapabilities) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_group_id(this: &MediaTrackCapabilities, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_height(this: &MediaTrackCapabilities) -> Option<ULongRange> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_height(this: &MediaTrackCapabilities, val: &ULongRange) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Get the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_latency(this: &MediaTrackCapabilities) -> Option<DoubleRange> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Change the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_latency(this: &MediaTrackCapabilities, val: &DoubleRange) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_noise_suppression(this: &MediaTrackCapabilities) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_noise_suppression(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `resizeMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_resize_mode(this: &MediaTrackCapabilities) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `resizeMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_resize_mode(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_sample_rate(this: &MediaTrackCapabilities) -> Option<ULongRange> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_sample_rate(this: &MediaTrackCapabilities, val: &ULongRange) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_sample_size(this: &MediaTrackCapabilities) -> Option<ULongRange> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_sample_size(this: &MediaTrackCapabilities, val: &ULongRange) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_width(this: &MediaTrackCapabilities) -> Option<ULongRange> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_width(this: &MediaTrackCapabilities, val: &ULongRange) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl MediaTrackCapabilities {
    #[doc = "Construct a new `MediaTrackCapabilities`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[deprecated = "Use `set_aspect_ratio()` instead."]
#[js::import = "None"]
pub fn aspect_ratio(&mut self, val: &DoubleRange) -> &mut Self { self.set_aspect_ratio(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_auto_gain_control()` instead."]
#[js::import = "None"]
pub fn auto_gain_control(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_auto_gain_control(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_background_blur()` instead."]
#[js::import = "None"]
pub fn background_blur(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_background_blur(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_channel_count()` instead."]
#[js::import = "None"]
pub fn channel_count(&mut self, val: &ULongRange) -> &mut Self { self.set_channel_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_device_id()` instead."]
#[js::import = "None"]
pub fn device_id(&mut self, val: &str) -> &mut Self { self.set_device_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_echo_cancellation()` instead."]
#[js::import = "None"]
pub fn echo_cancellation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_echo_cancellation(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_facing_mode()` instead."]
#[js::import = "None"]
pub fn facing_mode(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_facing_mode(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[deprecated = "Use `set_frame_rate()` instead."]
#[js::import = "None"]
pub fn frame_rate(&mut self, val: &DoubleRange) -> &mut Self { self.set_frame_rate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_group_id()` instead."]
#[js::import = "None"]
pub fn group_id(&mut self, val: &str) -> &mut Self { self.set_group_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_height()` instead."]
#[js::import = "None"]
pub fn height(&mut self, val: &ULongRange) -> &mut Self { self.set_height(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[deprecated = "Use `set_latency()` instead."]
#[js::import = "None"]
pub fn latency(&mut self, val: &DoubleRange) -> &mut Self { self.set_latency(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_noise_suppression()` instead."]
#[js::import = "None"]
pub fn noise_suppression(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_noise_suppression(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_resize_mode()` instead."]
#[js::import = "None"]
pub fn resize_mode(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_resize_mode(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_sample_rate()` instead."]
#[js::import = "None"]
pub fn sample_rate(&mut self, val: &ULongRange) -> &mut Self { self.set_sample_rate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_sample_size()` instead."]
#[js::import = "None"]
pub fn sample_size(&mut self, val: &ULongRange) -> &mut Self { self.set_sample_size(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[deprecated = "Use `set_width()` instead."]
#[js::import = "None"]
pub fn width(&mut self, val: &ULongRange) -> &mut Self { self.set_width(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MediaTrackCapabilities {
    fn default() -> Self {
        Self::new()
    }
}
