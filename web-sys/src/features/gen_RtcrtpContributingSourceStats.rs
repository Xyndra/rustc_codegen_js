#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcrtpContributingSourceStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcrtpContributingSourceStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcrtpContributingSourceStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcrtpContributingSourceStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcrtpContributingSourceStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcrtpContributingSourceStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcrtpContributingSourceStats`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcrtpContributingSourceStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcrtpContributingSourceStats`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcrtpContributingSourceStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `contributorSsrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
#[js::import = "None"]
pub fn get_contributor_ssrc(this: &RtcrtpContributingSourceStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `contributorSsrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
#[js::import = "None = $1"]
pub fn set_contributor_ssrc(this: &RtcrtpContributingSourceStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `inboundRtpStreamId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
#[js::import = "None"]
pub fn get_inbound_rtp_stream_id(this: &RtcrtpContributingSourceStats,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `inboundRtpStreamId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
#[js::import = "None = $1"]
pub fn set_inbound_rtp_stream_id(this: &RtcrtpContributingSourceStats, val: &str) { panic!("web-sys intrinsic") }
impl RtcrtpContributingSourceStats {
    #[doc = "Construct a new `RtcrtpContributingSourceStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcrtpContributingSourceStats`*"]
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
    #[deprecated = "Use `set_contributor_ssrc()` instead."]
#[js::import = "None"]
pub fn contributor_ssrc(&mut self, val: u32) -> &mut Self { self.set_contributor_ssrc(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_inbound_rtp_stream_id()` instead."]
#[js::import = "None"]
pub fn inbound_rtp_stream_id(&mut self, val: &str) -> &mut Self { self.set_inbound_rtp_stream_id(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcrtpContributingSourceStats {
    fn default() -> Self {
        Self::new()
    }
}
