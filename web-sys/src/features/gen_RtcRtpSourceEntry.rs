#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpSourceEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpSourceEntry(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
#[js::import = "None"]
pub fn get_audio_level(this: &RtcRtpSourceEntry) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
#[js::import = "None = $1"]
pub fn set_audio_level(this: &RtcRtpSourceEntry, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
#[js::import = "None"]
pub fn get_source(this: &RtcRtpSourceEntry) -> u32 { panic!("web-sys intrinsic") }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
#[js::import = "None = $1"]
pub fn set_source(this: &RtcRtpSourceEntry, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcRtpSourceEntry) -> f64 { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcRtpSourceEntry, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
#[js::import = "None"]
pub fn get_voice_activity_flag(this: &RtcRtpSourceEntry) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `voiceActivityFlag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`*"]
#[js::import = "None = $1"]
pub fn set_voice_activity_flag(this: &RtcRtpSourceEntry, val: Option<bool>) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[doc = "Get the `sourceType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"]
#[js::import = "None"]
pub fn get_source_type(this: &RtcRtpSourceEntry) -> RtcRtpSourceEntryType { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[doc = "Change the `sourceType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"]
#[js::import = "None = $1"]
pub fn set_source_type(this: &RtcRtpSourceEntry, val: RtcRtpSourceEntryType) { panic!("web-sys intrinsic") }
impl RtcRtpSourceEntry {
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[doc = "Construct a new `RtcRtpSourceEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpSourceEntry`, `RtcRtpSourceEntryType`*"]
#[js::import = "None"]
pub fn new(source: u32, timestamp: f64, source_type: RtcRtpSourceEntryType) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_source(source);
        ret.set_timestamp(timestamp);
        ret.set_source_type(source_type);
        ret
    }
    #[deprecated = "Use `set_audio_level()` instead."]
#[js::import = "None"]
pub fn audio_level(&mut self, val: f64) -> &mut Self { self.set_audio_level(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_source()` instead."]
#[js::import = "None"]
pub fn source(&mut self, val: u32) -> &mut Self { self.set_source(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
#[js::import = "None"]
pub fn timestamp(&mut self, val: f64) -> &mut Self { self.set_timestamp(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_voice_activity_flag()` instead."]
#[js::import = "None"]
pub fn voice_activity_flag(&mut self, val: Option<bool>) -> &mut Self { self.set_voice_activity_flag(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcRtpSourceEntryType")]
    #[deprecated = "Use `set_source_type()` instead."]
#[js::import = "None"]
pub fn source_type(&mut self, val: RtcRtpSourceEntryType) -> &mut Self { self.set_source_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
