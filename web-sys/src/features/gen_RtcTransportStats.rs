#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTransportStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcTransportStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcTransportStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcTransportStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcTransportStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcTransportStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcTransportStats`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcTransportStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsType`, `RtcTransportStats`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcTransportStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
#[js::import = "None"]
pub fn get_bytes_received(this: &RtcTransportStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
#[js::import = "None = $1"]
pub fn set_bytes_received(this: &RtcTransportStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
#[js::import = "None"]
pub fn get_bytes_sent(this: &RtcTransportStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
#[js::import = "None = $1"]
pub fn set_bytes_sent(this: &RtcTransportStats, val: u32) { panic!("web-sys intrinsic") }
impl RtcTransportStats {
    #[doc = "Construct a new `RtcTransportStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTransportStats`*"]
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
pub fn bytes_received(&mut self, val: u32) -> &mut Self { self.set_bytes_received(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_bytes_sent()` instead."]
#[js::import = "None"]
pub fn bytes_sent(&mut self, val: u32) -> &mut Self { self.set_bytes_sent(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcTransportStats {
    fn default() -> Self {
        Self::new()
    }
}
