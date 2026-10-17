#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcInboundRtpStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcInboundRtpStreamStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcInboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcInboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcInboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`, `RtcStatsType`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcInboundRtpStreamStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`, `RtcStatsType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcInboundRtpStreamStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_bitrate_mean(this: &RtcInboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bitrateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_bitrate_mean(this: &RtcInboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_bitrate_std_dev(this: &RtcInboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bitrateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_bitrate_std_dev(this: &RtcInboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_codec_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `codecId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_codec_id(this: &RtcInboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_fir_count(this: &RtcInboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `firCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_fir_count(this: &RtcInboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_framerate_mean(this: &RtcInboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framerateMean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_framerate_mean(this: &RtcInboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_framerate_std_dev(this: &RtcInboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framerateStdDev` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_framerate_std_dev(this: &RtcInboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_is_remote(this: &RtcInboundRtpStreamStats) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `isRemote` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_is_remote(this: &RtcInboundRtpStreamStats, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_media_track_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `mediaTrackId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_media_track_id(this: &RtcInboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_media_type(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `mediaType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_media_type(this: &RtcInboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_nack_count(this: &RtcInboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `nackCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_nack_count(this: &RtcInboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_pli_count(this: &RtcInboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `pliCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_pli_count(this: &RtcInboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_remote_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `remoteId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_remote_id(this: &RtcInboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_ssrc(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_ssrc(this: &RtcInboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_transport_id(this: &RtcInboundRtpStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_transport_id(this: &RtcInboundRtpStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_bytes_received(this: &RtcInboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_bytes_received(this: &RtcInboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `discardedPackets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_discarded_packets(this: &RtcInboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `discardedPackets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_discarded_packets(this: &RtcInboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_frames_decoded(this: &RtcInboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_frames_decoded(this: &RtcInboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `jitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_jitter(this: &RtcInboundRtpStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `jitter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_jitter(this: &RtcInboundRtpStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `packetsLost` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_packets_lost(this: &RtcInboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `packetsLost` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_packets_lost(this: &RtcInboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `packetsReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_packets_received(this: &RtcInboundRtpStreamStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `packetsReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_packets_received(this: &RtcInboundRtpStreamStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `roundTripTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None"]
pub fn get_round_trip_time(this: &RtcInboundRtpStreamStats) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `roundTripTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_round_trip_time(this: &RtcInboundRtpStreamStats, val: i32) { panic!("web-sys intrinsic") }
impl RtcInboundRtpStreamStats {
    #[doc = "Construct a new `RtcInboundRtpStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcInboundRtpStreamStats`*"]
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
    #[deprecated = "Use `set_bytes_received()` instead."]
#[js::import = "None"]
pub fn bytes_received(&mut self, val: f64) -> &mut Self { self.set_bytes_received(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_discarded_packets()` instead."]
#[js::import = "None"]
pub fn discarded_packets(&mut self, val: u32) -> &mut Self { self.set_discarded_packets(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_frames_decoded()` instead."]
#[js::import = "None"]
pub fn frames_decoded(&mut self, val: u32) -> &mut Self { self.set_frames_decoded(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_jitter()` instead."]
#[js::import = "None"]
pub fn jitter(&mut self, val: f64) -> &mut Self { self.set_jitter(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_packets_lost()` instead."]
#[js::import = "None"]
pub fn packets_lost(&mut self, val: u32) -> &mut Self { self.set_packets_lost(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_packets_received()` instead."]
#[js::import = "None"]
pub fn packets_received(&mut self, val: u32) -> &mut Self { self.set_packets_received(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_round_trip_time()` instead."]
#[js::import = "None"]
pub fn round_trip_time(&mut self, val: i32) -> &mut Self { self.set_round_trip_time(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcInboundRtpStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
