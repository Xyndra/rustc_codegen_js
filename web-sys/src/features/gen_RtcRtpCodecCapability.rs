#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpCodecCapability` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpCodecCapability(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::import = "None"]
pub fn get_channels(this: &RtcRtpCodecCapability) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `channels` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::import = "None = $1"]
pub fn set_channels(this: &RtcRtpCodecCapability, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::import = "None"]
pub fn get_clock_rate(this: &RtcRtpCodecCapability) -> u32 { panic!("web-sys intrinsic") }
    #[doc = "Change the `clockRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::import = "None = $1"]
pub fn set_clock_rate(this: &RtcRtpCodecCapability, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::import = "None"]
pub fn get_mime_type(this: &RtcRtpCodecCapability) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::import = "None = $1"]
pub fn set_mime_type(this: &RtcRtpCodecCapability, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::import = "None"]
pub fn get_sdp_fmtp_line(this: &RtcRtpCodecCapability) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sdpFmtpLine` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::import = "None = $1"]
pub fn set_sdp_fmtp_line(this: &RtcRtpCodecCapability, val: &str) { panic!("web-sys intrinsic") }
impl RtcRtpCodecCapability {
    #[doc = "Construct a new `RtcRtpCodecCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCodecCapability`*"]
#[js::import = "None"]
pub fn new(clock_rate: u32, mime_type: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_clock_rate(clock_rate);
        ret.set_mime_type(mime_type);
        ret
    }
    #[deprecated = "Use `set_channels()` instead."]
#[js::import = "None"]
pub fn channels(&mut self, val: u16) -> &mut Self { self.set_channels(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_clock_rate()` instead."]
#[js::import = "None"]
pub fn clock_rate(&mut self, val: u32) -> &mut Self { self.set_clock_rate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_mime_type()` instead."]
#[js::import = "None"]
pub fn mime_type(&mut self, val: &str) -> &mut Self { self.set_mime_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_sdp_fmtp_line()` instead."]
#[js::import = "None"]
pub fn sdp_fmtp_line(&mut self, val: &str) -> &mut Self { self.set_sdp_fmtp_line(val) { panic!("web-sys intrinsic") }
        self
    }
}
