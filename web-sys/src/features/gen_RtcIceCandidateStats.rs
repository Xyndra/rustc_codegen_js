#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidateStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIceCandidateStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcIceCandidateStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcIceCandidateStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcIceCandidateStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcIceCandidateStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsType`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcIceCandidateStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcIceCandidateStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `candidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None"]
pub fn get_candidate_id(this: &RtcIceCandidateStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `candidateId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None = $1"]
pub fn set_candidate_id(this: &RtcIceCandidateStats, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[doc = "Get the `candidateType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsIceCandidateType`*"]
#[js::import = "None"]
pub fn get_candidate_type(this: &RtcIceCandidateStats) -> Option<RtcStatsIceCandidateType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[doc = "Change the `candidateType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`, `RtcStatsIceCandidateType`*"]
#[js::import = "None = $1"]
pub fn set_candidate_type(this: &RtcIceCandidateStats, val: RtcStatsIceCandidateType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None"]
pub fn get_component_id(this: &RtcIceCandidateStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `componentId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None = $1"]
pub fn set_component_id(this: &RtcIceCandidateStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ipAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None"]
pub fn get_ip_address(this: &RtcIceCandidateStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ipAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None = $1"]
pub fn set_ip_address(this: &RtcIceCandidateStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `portNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None"]
pub fn get_port_number(this: &RtcIceCandidateStats) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `portNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None = $1"]
pub fn set_port_number(this: &RtcIceCandidateStats, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `transport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None"]
pub fn get_transport(this: &RtcIceCandidateStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `transport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
#[js::import = "None = $1"]
pub fn set_transport(this: &RtcIceCandidateStats, val: &str) { panic!("web-sys intrinsic") }
impl RtcIceCandidateStats {
    #[doc = "Construct a new `RtcIceCandidateStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateStats`*"]
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
    #[deprecated = "Use `set_candidate_id()` instead."]
#[js::import = "None"]
pub fn candidate_id(&mut self, val: &str) -> &mut Self { self.set_candidate_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcStatsIceCandidateType")]
    #[deprecated = "Use `set_candidate_type()` instead."]
#[js::import = "None"]
pub fn candidate_type(&mut self, val: RtcStatsIceCandidateType) -> &mut Self { self.set_candidate_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_component_id()` instead."]
#[js::import = "None"]
pub fn component_id(&mut self, val: &str) -> &mut Self { self.set_component_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ip_address()` instead."]
#[js::import = "None"]
pub fn ip_address(&mut self, val: &str) -> &mut Self { self.set_ip_address(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_port_number()` instead."]
#[js::import = "None"]
pub fn port_number(&mut self, val: i32) -> &mut Self { self.set_port_number(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_transport()` instead."]
#[js::import = "None"]
pub fn transport(&mut self, val: &str) -> &mut Self { self.set_transport(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcIceCandidateStats {
    fn default() -> Self {
        Self::new()
    }
}
