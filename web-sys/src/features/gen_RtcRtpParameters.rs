#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
#[js::import = "None"]
pub fn get_codecs(this: &RtcRtpParameters) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
#[js::import = "None = $1"]
pub fn set_codecs(this: &RtcRtpParameters, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `encodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
#[js::import = "None"]
pub fn get_encodings(this: &RtcRtpParameters) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `encodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
#[js::import = "None = $1"]
pub fn set_encodings(this: &RtcRtpParameters, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
#[js::import = "None"]
pub fn get_header_extensions(this: &RtcRtpParameters) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
#[js::import = "None = $1"]
pub fn set_header_extensions(this: &RtcRtpParameters, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtcpParameters")]
    #[doc = "Get the `rtcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`, `RtcRtpParameters`*"]
#[js::import = "None"]
pub fn get_rtcp(this: &RtcRtpParameters) -> Option<RtcRtcpParameters> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtcpParameters")]
    #[doc = "Change the `rtcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtcpParameters`, `RtcRtpParameters`*"]
#[js::import = "None = $1"]
pub fn set_rtcp(this: &RtcRtpParameters, val: &RtcRtcpParameters) { panic!("web-sys intrinsic") }
impl RtcRtpParameters {
    #[doc = "Construct a new `RtcRtpParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpParameters`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_codecs()` instead."]
#[js::import = "None"]
pub fn codecs(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_codecs(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_encodings()` instead."]
#[js::import = "None"]
pub fn encodings(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_encodings(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_header_extensions()` instead."]
#[js::import = "None"]
pub fn header_extensions(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_header_extensions(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcRtcpParameters")]
    #[deprecated = "Use `set_rtcp()` instead."]
#[js::import = "None"]
pub fn rtcp(&mut self, val: &RtcRtcpParameters) -> &mut Self { self.set_rtcp(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcRtpParameters {
    fn default() -> Self {
        Self::new()
    }
}
