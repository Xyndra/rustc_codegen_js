#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpContributingSource` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpContributingSource(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
#[js::import = "None"]
pub fn get_audio_level(this: &RtcRtpContributingSource) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
#[js::import = "None = $1"]
pub fn set_audio_level(this: &RtcRtpContributingSource, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
#[js::import = "None"]
pub fn get_source(this: &RtcRtpContributingSource) -> u32 { panic!("web-sys intrinsic") }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
#[js::import = "None = $1"]
pub fn set_source(this: &RtcRtpContributingSource, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &RtcRtpContributingSource) -> f64 { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &RtcRtpContributingSource, val: f64) { panic!("web-sys intrinsic") }
impl RtcRtpContributingSource {
    #[doc = "Construct a new `RtcRtpContributingSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpContributingSource`*"]
#[js::import = "None"]
pub fn new(source: u32, timestamp: f64) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_source(source);
        ret.set_timestamp(timestamp);
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
}
