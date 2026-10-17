#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcMediaStreamTrackStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcMediaStreamTrackStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcMediaStreamTrackStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcMediaStreamTrackStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcMediaStreamTrackStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcMediaStreamTrackStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcMediaStreamTrackStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcMediaStreamTrackStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_audio_level(this: &RtcMediaStreamTrackStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_audio_level(this: &RtcMediaStreamTrackStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_echo_return_loss(this: &RtcMediaStreamTrackStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_echo_return_loss(this: &RtcMediaStreamTrackStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_echo_return_loss_enhancement(this: &RtcMediaStreamTrackStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_echo_return_loss_enhancement(this: &RtcMediaStreamTrackStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_frame_height(this: &RtcMediaStreamTrackStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_frame_height(this: &RtcMediaStreamTrackStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_frame_width(this: &RtcMediaStreamTrackStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_frame_width(this: &RtcMediaStreamTrackStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_frames_corrupted(this: &RtcMediaStreamTrackStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_frames_corrupted(this: &RtcMediaStreamTrackStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_frames_decoded(this: &RtcMediaStreamTrackStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_frames_decoded(this: &RtcMediaStreamTrackStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_frames_dropped(this: &RtcMediaStreamTrackStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_frames_dropped(this: &RtcMediaStreamTrackStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_frames_per_second(this: &RtcMediaStreamTrackStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_frames_per_second(this: &RtcMediaStreamTrackStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_frames_received(this: &RtcMediaStreamTrackStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_frames_received(this: &RtcMediaStreamTrackStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_frames_sent(this: &RtcMediaStreamTrackStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_frames_sent(this: &RtcMediaStreamTrackStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_remote_source(this: &RtcMediaStreamTrackStats) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_remote_source(this: &RtcMediaStreamTrackStats, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_ssrc_ids(this: &RtcMediaStreamTrackStats) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_ssrc_ids(this: &RtcMediaStreamTrackStats, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn get_track_identifier(this: &RtcMediaStreamTrackStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None = $1"]
pub fn set_track_identifier(this: &RtcMediaStreamTrackStats, val: &str) { panic!("web-sys intrinsic") }
impl RtcMediaStreamTrackStats {
    #[doc = "Construct a new `RtcMediaStreamTrackStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
#[js::import = "None"]
pub fn id(&mut self, val: &str) -> &mut Self { self.set_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
#[js::import = "None"]
pub fn timestamp(&mut self, val: f64) -> &mut Self { self.set_timestamp(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: RtcStatsType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_audio_level()` instead."]
#[js::import = "None"]
pub fn audio_level(&mut self, val: f64) -> &mut Self { self.set_audio_level(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_echo_return_loss()` instead."]
#[js::import = "None"]
pub fn echo_return_loss(&mut self, val: f64) -> &mut Self { self.set_echo_return_loss(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_echo_return_loss_enhancement()` instead."]
#[js::import = "None"]
pub fn echo_return_loss_enhancement(&mut self, val: f64) -> &mut Self { self.set_echo_return_loss_enhancement(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frame_height()` instead."]
#[js::import = "None"]
pub fn frame_height(&mut self, val: u32) -> &mut Self { self.set_frame_height(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frame_width()` instead."]
#[js::import = "None"]
pub fn frame_width(&mut self, val: u32) -> &mut Self { self.set_frame_width(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frames_corrupted()` instead."]
#[js::import = "None"]
pub fn frames_corrupted(&mut self, val: u32) -> &mut Self { self.set_frames_corrupted(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frames_decoded()` instead."]
#[js::import = "None"]
pub fn frames_decoded(&mut self, val: u32) -> &mut Self { self.set_frames_decoded(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frames_dropped()` instead."]
#[js::import = "None"]
pub fn frames_dropped(&mut self, val: u32) -> &mut Self { self.set_frames_dropped(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frames_per_second()` instead."]
#[js::import = "None"]
pub fn frames_per_second(&mut self, val: f64) -> &mut Self { self.set_frames_per_second(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frames_received()` instead."]
#[js::import = "None"]
pub fn frames_received(&mut self, val: u32) -> &mut Self { self.set_frames_received(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frames_sent()` instead."]
#[js::import = "None"]
pub fn frames_sent(&mut self, val: u32) -> &mut Self { self.set_frames_sent(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_remote_source()` instead."]
#[js::import = "None"]
pub fn remote_source(&mut self, val: bool) -> &mut Self { self.set_remote_source(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ssrc_ids()` instead."]
#[js::import = "None"]
pub fn ssrc_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_ssrc_ids(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_track_identifier()` instead."]
#[js::import = "None"]
pub fn track_identifier(&mut self, val: &str) -> &mut Self { self.set_track_identifier(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcMediaStreamTrackStats {
    fn default() -> Self {
        Self::new()
    }
}
