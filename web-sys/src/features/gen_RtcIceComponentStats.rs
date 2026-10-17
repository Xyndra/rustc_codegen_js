#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceComponentStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIceComponentStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None"]
pub fn get_id(this: &RtcIceComponentStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &RtcIceComponentStats, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcIceComponentStats) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcIceComponentStats, val: f64) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`, `RtcStatsType`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcIceComponentStats) -> Option<RtcStatsType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`, `RtcStatsType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcIceComponentStats, val: RtcStatsType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `activeConnection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None"]
pub fn get_active_connection(this: &RtcIceComponentStats) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `activeConnection` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None = $1"]
pub fn set_active_connection(this: &RtcIceComponentStats, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None"]
pub fn get_bytes_received(this: &RtcIceComponentStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None = $1"]
pub fn set_bytes_received(this: &RtcIceComponentStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None"]
pub fn get_bytes_sent(this: &RtcIceComponentStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None = $1"]
pub fn set_bytes_sent(this: &RtcIceComponentStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `component` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None"]
pub fn get_component(this: &RtcIceComponentStats) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `component` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None = $1"]
pub fn set_component(this: &RtcIceComponentStats, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None"]
pub fn get_transport_id(this: &RtcIceComponentStats) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `transportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
#[js::import = "None = $1"]
pub fn set_transport_id(this: &RtcIceComponentStats, val: &str) { panic!("web-sys intrinsic") }
impl RtcIceComponentStats {
    #[doc = "Construct a new `RtcIceComponentStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceComponentStats`*"]
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
    #[deprecated = "Use `set_active_connection()` instead."]
#[js::import = "None"]
pub fn active_connection(&mut self, val: bool) -> &mut Self { self.set_active_connection(val) { panic!("web-sys intrinsic") }
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
    #[deprecated = "Use `set_component()` instead."]
#[js::import = "None"]
pub fn component(&mut self, val: i32) -> &mut Self { self.set_component(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_transport_id()` instead."]
#[js::import = "None"]
pub fn transport_id(&mut self, val: &str) -> &mut Self { self.set_transport_id(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcIceComponentStats {
    fn default() -> Self {
        Self::new()
    }
}
