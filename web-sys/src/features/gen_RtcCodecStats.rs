#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcCodecStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcCodecStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcCodecStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcCodecStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcCodecStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcCodecStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`, `RtcStatsType`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcCodecStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`, `RtcStatsType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcCodecStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None"]
pub fn get_channels(this: &RtcCodecStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None = $1"]
pub fn set_channels(this: &RtcCodecStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None"]
pub fn get_clock_rate(this: &RtcCodecStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None = $1"]
pub fn set_clock_rate(this: &RtcCodecStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None"]
pub fn get_codec(this: &RtcCodecStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `codec` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None = $1"]
pub fn set_codec(this: &RtcCodecStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None"]
pub fn get_parameters(this: &RtcCodecStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None = $1"]
pub fn set_parameters(this: &RtcCodecStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None"]
pub fn get_payload_type(this: &RtcCodecStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `payloadType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
#[js::import = "None = $1"]
pub fn set_payload_type(this: &RtcCodecStats, val: u32) { panic!("web-sys intrinsic") }
impl RtcCodecStats {
    #[doc = "Construct a new `RtcCodecStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcCodecStats`*"]
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
    #[deprecated = "Use `set_channels()` instead."]
#[js::import = "None"]
pub fn channels(&mut self, val: u32) -> &mut Self { self.set_channels(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_clock_rate()` instead."]
#[js::import = "None"]
pub fn clock_rate(&mut self, val: u32) -> &mut Self { self.set_clock_rate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_codec()` instead."]
#[js::import = "None"]
pub fn codec(&mut self, val: &str) -> &mut Self { self.set_codec(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_parameters()` instead."]
#[js::import = "None"]
pub fn parameters(&mut self, val: &str) -> &mut Self { self.set_parameters(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_payload_type()` instead."]
#[js::import = "None"]
pub fn payload_type(&mut self, val: u32) -> &mut Self { self.set_payload_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcCodecStats {
    fn default() -> Self {
        Self::new()
    }
}
