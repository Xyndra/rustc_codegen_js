#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcMediaStreamStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcMediaStreamStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcMediaStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcMediaStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcMediaStreamStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcMediaStreamStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`, `RtcStatsType`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcMediaStreamStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`, `RtcStatsType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcMediaStreamStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `streamIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
#[js::import = "None"]
pub fn get_stream_identifier(this: &RtcMediaStreamStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `streamIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_stream_identifier(this: &RtcMediaStreamStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `trackIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
#[js::import = "None"]
pub fn get_track_ids(this: &RtcMediaStreamStats) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `trackIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
#[js::import = "None = $1"]
pub fn set_track_ids(this: &RtcMediaStreamStats, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl RtcMediaStreamStats {
    #[doc = "Construct a new `RtcMediaStreamStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamStats`*"]
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
    #[deprecated = "Use `set_stream_identifier()` instead."]
#[js::import = "None"]
pub fn stream_identifier(&mut self, val: &str) -> &mut Self { self.set_stream_identifier(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_track_ids()` instead."]
#[js::import = "None"]
pub fn track_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_track_ids(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcMediaStreamStats {
    fn default() -> Self {
        Self::new()
    }
}
