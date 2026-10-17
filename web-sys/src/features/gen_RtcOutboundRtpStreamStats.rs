#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcOutboundRtpStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcOutboundRtpStreamStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcOutboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcOutboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcOutboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`, `RtcStatsType`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcOutboundRtpStreamStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`, `RtcStatsType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcOutboundRtpStreamStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_bitrate_mean(this: &RtcOutboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_bitrate_mean(this: &RtcOutboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_bitrate_std_dev(this: &RtcOutboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_bitrate_std_dev(this: &RtcOutboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_codec_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_codec_id(this: &RtcOutboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_fir_count(this: &RtcOutboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_fir_count(this: &RtcOutboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_framerate_mean(this: &RtcOutboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_framerate_mean(this: &RtcOutboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_framerate_std_dev(this: &RtcOutboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_framerate_std_dev(this: &RtcOutboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_is_remote(this: &RtcOutboundRtpStreamStats) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_is_remote(this: &RtcOutboundRtpStreamStats, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_media_track_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_media_track_id(this: &RtcOutboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_media_type(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_media_type(this: &RtcOutboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_nack_count(this: &RtcOutboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_nack_count(this: &RtcOutboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_pli_count(this: &RtcOutboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_pli_count(this: &RtcOutboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_remote_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_remote_id(this: &RtcOutboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_ssrc(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_ssrc(this: &RtcOutboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_transport_id(this: &RtcOutboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_transport_id(this: &RtcOutboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_bytes_sent(this: &RtcOutboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_bytes_sent(this: &RtcOutboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `droppedFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_dropped_frames(this: &RtcOutboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `droppedFrames` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_dropped_frames(this: &RtcOutboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framesEncoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_frames_encoded(this: &RtcOutboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framesEncoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_frames_encoded(this: &RtcOutboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_packets_sent(this: &RtcOutboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `packetsSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_packets_sent(this: &RtcOutboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `targetBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_target_bitrate(this: &RtcOutboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `targetBitrate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_target_bitrate(this: &RtcOutboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
impl RtcOutboundRtpStreamStats {
    #[doc = "Construct a new `RtcOutboundRtpStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcOutboundRtpStreamStats`*"]
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
    #[deprecated = "Use `set_bitrate_mean()` instead."]
#[js::import = "None"]
pub fn bitrate_mean(&mut self, val: f64) -> &mut Self { self.set_bitrate_mean(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_bitrate_std_dev()` instead."]
#[js::import = "None"]
pub fn bitrate_std_dev(&mut self, val: f64) -> &mut Self { self.set_bitrate_std_dev(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_codec_id()` instead."]
#[js::import = "None"]
pub fn codec_id(&mut self, val: &str) -> &mut Self { self.set_codec_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_fir_count()` instead."]
#[js::import = "None"]
pub fn fir_count(&mut self, val: u32) -> &mut Self { self.set_fir_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_framerate_mean()` instead."]
#[js::import = "None"]
pub fn framerate_mean(&mut self, val: f64) -> &mut Self { self.set_framerate_mean(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_framerate_std_dev()` instead."]
#[js::import = "None"]
pub fn framerate_std_dev(&mut self, val: f64) -> &mut Self { self.set_framerate_std_dev(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_is_remote()` instead."]
#[js::import = "None"]
pub fn is_remote(&mut self, val: bool) -> &mut Self { self.set_is_remote(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_media_track_id()` instead."]
#[js::import = "None"]
pub fn media_track_id(&mut self, val: &str) -> &mut Self { self.set_media_track_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_media_type()` instead."]
#[js::import = "None"]
pub fn media_type(&mut self, val: &str) -> &mut Self { self.set_media_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_nack_count()` instead."]
#[js::import = "None"]
pub fn nack_count(&mut self, val: u32) -> &mut Self { self.set_nack_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_pli_count()` instead."]
#[js::import = "None"]
pub fn pli_count(&mut self, val: u32) -> &mut Self { self.set_pli_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_remote_id()` instead."]
#[js::import = "None"]
pub fn remote_id(&mut self, val: &str) -> &mut Self { self.set_remote_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ssrc()` instead."]
#[js::import = "None"]
pub fn ssrc(&mut self, val: &str) -> &mut Self { self.set_ssrc(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_transport_id()` instead."]
#[js::import = "None"]
pub fn transport_id(&mut self, val: &str) -> &mut Self { self.set_transport_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_bytes_sent()` instead."]
#[js::import = "None"]
pub fn bytes_sent(&mut self, val: f64) -> &mut Self { self.set_bytes_sent(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_dropped_frames()` instead."]
#[js::import = "None"]
pub fn dropped_frames(&mut self, val: u32) -> &mut Self { self.set_dropped_frames(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frames_encoded()` instead."]
#[js::import = "None"]
pub fn frames_encoded(&mut self, val: u32) -> &mut Self { self.set_frames_encoded(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_packets_sent()` instead."]
#[js::import = "None"]
pub fn packets_sent(&mut self, val: u32) -> &mut Self { self.set_packets_sent(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_target_bitrate()` instead."]
#[js::import = "None"]
pub fn target_bitrate(&mut self, val: f64) -> &mut Self { self.set_target_bitrate(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcOutboundRtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
