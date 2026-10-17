#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidatePairStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIceCandidatePairStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcIceCandidatePairStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcIceCandidatePairStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcIceCandidatePairStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcIceCandidatePairStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsType`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcIceCandidatePairStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcIceCandidatePairStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_bytes_received(this: &RtcIceCandidatePairStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_bytes_received(this: &RtcIceCandidatePairStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_bytes_sent(this: &RtcIceCandidatePairStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_bytes_sent(this: &RtcIceCandidatePairStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_component_id(this: &RtcIceCandidatePairStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_component_id(this: &RtcIceCandidatePairStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `lastPacketReceivedTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_last_packet_received_timestamp(this: &RtcIceCandidatePairStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lastPacketReceivedTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_last_packet_received_timestamp(this: &RtcIceCandidatePairStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `lastPacketSentTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_last_packet_sent_timestamp(this: &RtcIceCandidatePairStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lastPacketSentTimestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_last_packet_sent_timestamp(this: &RtcIceCandidatePairStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `localCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_local_candidate_id(this: &RtcIceCandidatePairStats,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `localCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_local_candidate_id(this: &RtcIceCandidatePairStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `nominated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_nominated(this: &RtcIceCandidatePairStats) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `nominated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_nominated(this: &RtcIceCandidatePairStats, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_priority(this: &RtcIceCandidatePairStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `priority` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_priority(this: &RtcIceCandidatePairStats, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_readable(this: &RtcIceCandidatePairStats) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_readable(this: &RtcIceCandidatePairStats, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `remoteCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_remote_candidate_id(this: &RtcIceCandidatePairStats,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `remoteCandidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_remote_candidate_id(this: &RtcIceCandidatePairStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `selected` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_selected(this: &RtcIceCandidatePairStats) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `selected` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_selected(this: &RtcIceCandidatePairStats, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsIceCandidatePairState`*"]
#[js::import = "None"]
pub fn get_state(this: &RtcIceCandidatePairStats) -> Option<RtcStatsIceCandidatePairState> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`, `RtcStatsIceCandidatePairState`*"]
#[js::import = "None = $1"]
pub fn set_state(this: &RtcIceCandidatePairStats, val: RtcStatsIceCandidatePairState) { panic!("web-sys intrinsic") }
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_transport_id(this: &RtcIceCandidatePairStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_transport_id(this: &RtcIceCandidatePairStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None"]
pub fn get_writable(this: &RtcIceCandidatePairStats) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
#[js::import = "None = $1"]
pub fn set_writable(this: &RtcIceCandidatePairStats, val: bool) { panic!("web-sys intrinsic") }
impl RtcIceCandidatePairStats {
    #[doc = "Construct a new `RtcIceCandidatePairStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidatePairStats`*"]
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
    #[deprecated = "Use `set_bytes_received()` instead."]
#[js::import = "None"]
pub fn bytes_received(&mut self, val: f64) -> &mut Self { self.set_bytes_received(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_bytes_sent()` instead."]
#[js::import = "None"]
pub fn bytes_sent(&mut self, val: f64) -> &mut Self { self.set_bytes_sent(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_component_id()` instead."]
#[js::import = "None"]
pub fn component_id(&mut self, val: u32) -> &mut Self { self.set_component_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_last_packet_received_timestamp()` instead."]
#[js::import = "None"]
pub fn last_packet_received_timestamp(&mut self, val: f64) -> &mut Self { self.set_last_packet_received_timestamp(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_last_packet_sent_timestamp()` instead."]
#[js::import = "None"]
pub fn last_packet_sent_timestamp(&mut self, val: f64) -> &mut Self { self.set_last_packet_sent_timestamp(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_local_candidate_id()` instead."]
#[js::import = "None"]
pub fn local_candidate_id(&mut self, val: &str) -> &mut Self { self.set_local_candidate_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_nominated()` instead."]
#[js::import = "None"]
pub fn nominated(&mut self, val: bool) -> &mut Self { self.set_nominated(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_priority()` instead."]
#[js::import = "None"]
pub fn priority(&mut self, val: f64) -> &mut Self { self.set_priority(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_readable()` instead."]
#[js::import = "None"]
pub fn readable(&mut self, val: bool) -> &mut Self { self.set_readable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_remote_candidate_id()` instead."]
#[js::import = "None"]
pub fn remote_candidate_id(&mut self, val: &str) -> &mut Self { self.set_remote_candidate_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_selected()` instead."]
#[js::import = "None"]
pub fn selected(&mut self, val: bool) -> &mut Self { self.set_selected(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcStatsIceCandidatePairState")]
    #[deprecated = "Use `set_state()` instead."]
#[js::import = "None"]
pub fn state(&mut self, val: RtcStatsIceCandidatePairState) -> &mut Self { self.set_state(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_transport_id()` instead."]
#[js::import = "None"]
pub fn transport_id(&mut self, val: &str) -> &mut Self { self.set_transport_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_writable()` instead."]
#[js::import = "None"]
pub fn writable(&mut self, val: bool) -> &mut Self { self.set_writable(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcIceCandidatePairStats {
    fn default() -> Self {
        Self::new()
    }
}
