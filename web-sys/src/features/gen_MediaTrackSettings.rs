#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackSettings` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaTrackSettings(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn get_auto_gain_control(this: &MediaTrackSettings) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None = $1"]
pub fn set_auto_gain_control(this: &MediaTrackSettings, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn get_channel_count(this: &MediaTrackSettings) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None = $1"]
pub fn set_channel_count(this: &MediaTrackSettings, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn get_device_id(this: &MediaTrackSettings) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None = $1"]
pub fn set_device_id(this: &MediaTrackSettings, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn get_echo_cancellation(this: &MediaTrackSettings) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None = $1"]
pub fn set_echo_cancellation(this: &MediaTrackSettings, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn get_facing_mode(this: &MediaTrackSettings) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None = $1"]
pub fn set_facing_mode(this: &MediaTrackSettings, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn get_frame_rate(this: &MediaTrackSettings) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None = $1"]
pub fn set_frame_rate(this: &MediaTrackSettings, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn get_height(this: &MediaTrackSettings) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None = $1"]
pub fn set_height(this: &MediaTrackSettings, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn get_noise_suppression(this: &MediaTrackSettings) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None = $1"]
pub fn set_noise_suppression(this: &MediaTrackSettings, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn get_width(this: &MediaTrackSettings) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None = $1"]
pub fn set_width(this: &MediaTrackSettings, val: i32) { panic!("web-sys intrinsic") }
impl MediaTrackSettings {
    #[doc = "Construct a new `MediaTrackSettings`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackSettings`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_auto_gain_control()` instead."]
#[js::import = "None"]
pub fn auto_gain_control(&mut self, val: bool) -> &mut Self { self.set_auto_gain_control(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_channel_count()` instead."]
#[js::import = "None"]
pub fn channel_count(&mut self, val: i32) -> &mut Self { self.set_channel_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_device_id()` instead."]
#[js::import = "None"]
pub fn device_id(&mut self, val: &str) -> &mut Self { self.set_device_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_echo_cancellation()` instead."]
#[js::import = "None"]
pub fn echo_cancellation(&mut self, val: bool) -> &mut Self { self.set_echo_cancellation(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_facing_mode()` instead."]
#[js::import = "None"]
pub fn facing_mode(&mut self, val: &str) -> &mut Self { self.set_facing_mode(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frame_rate()` instead."]
#[js::import = "None"]
pub fn frame_rate(&mut self, val: f64) -> &mut Self { self.set_frame_rate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_height()` instead."]
#[js::import = "None"]
pub fn height(&mut self, val: i32) -> &mut Self { self.set_height(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_noise_suppression()` instead."]
#[js::import = "None"]
pub fn noise_suppression(&mut self, val: bool) -> &mut Self { self.set_noise_suppression(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_width()` instead."]
#[js::import = "None"]
pub fn width(&mut self, val: i32) -> &mut Self { self.set_width(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for MediaTrackSettings {
    fn default() -> Self {
        Self::new()
    }
}
