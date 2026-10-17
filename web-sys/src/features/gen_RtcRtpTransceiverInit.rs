#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpTransceiverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpTransceiverInit(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`, `RtcRtpTransceiverInit`*"]
#[js::import = "None"]
pub fn get_direction(this: &RtcRtpTransceiverInit) -> Option<RtcRtpTransceiverDirection> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`, `RtcRtpTransceiverInit`*"]
#[js::import = "None = $1"]
pub fn set_direction(this: &RtcRtpTransceiverInit, val: RtcRtpTransceiverDirection) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sendEncodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
#[js::import = "None"]
pub fn get_send_encodings(this: &RtcRtpTransceiverInit) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sendEncodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
#[js::import = "None = $1"]
pub fn set_send_encodings(this: &RtcRtpTransceiverInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
#[js::import = "None"]
pub fn get_streams(this: &RtcRtpTransceiverInit) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
#[js::import = "None = $1"]
pub fn set_streams(this: &RtcRtpTransceiverInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl RtcRtpTransceiverInit {
    #[doc = "Construct a new `RtcRtpTransceiverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[deprecated = "Use `set_direction()` instead."]
#[js::import = "None"]
pub fn direction(&mut self, val: RtcRtpTransceiverDirection) -> &mut Self { self.set_direction(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_send_encodings()` instead."]
#[js::import = "None"]
pub fn send_encodings(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_send_encodings(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_streams()` instead."]
#[js::import = "None"]
pub fn streams(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_streams(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcRtpTransceiverInit {
    fn default() -> Self {
        Self::new()
    }
}
