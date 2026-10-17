#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpCapabilities` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpCapabilities(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
#[js::import = "None"]
pub fn get_codecs(this: &RtcRtpCapabilities) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[doc = "Change the `codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
#[js::import = "None = $1"]
pub fn set_codecs(this: &RtcRtpCapabilities, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
#[js::import = "None"]
pub fn get_header_extensions(this: &RtcRtpCapabilities) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[doc = "Change the `headerExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
#[js::import = "None = $1"]
pub fn set_header_extensions(this: &RtcRtpCapabilities, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl RtcRtpCapabilities {
    #[doc = "Construct a new `RtcRtpCapabilities`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpCapabilities`*"]
#[js::import = "None"]
pub fn new(codecs: &::wasm_bindgen::JsValue, header_extensions: &::wasm_bindgen::JsValue,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_codecs(codecs);
        ret.set_header_extensions(header_extensions);
        ret
    }
    #[deprecated = "Use `set_codecs()` instead."]
#[js::import = "None"]
pub fn codecs(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_codecs(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_header_extensions()` instead."]
#[js::import = "None"]
pub fn header_extensions(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_header_extensions(val) { panic!("web-sys intrinsic") }
        self
    }
}
