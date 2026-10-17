#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceCandidateInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIceCandidateInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
#[js::import = "None"]
pub fn get_candidate(this: &RtcIceCandidateInit) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
#[js::import = "None = $1"]
pub fn set_candidate(this: &RtcIceCandidateInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
#[js::import = "None"]
pub fn get_sdp_m_line_index(this: &RtcIceCandidateInit) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sdpMLineIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
#[js::import = "None = $1"]
pub fn set_sdp_m_line_index(this: &RtcIceCandidateInit, val: Option<u16>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
#[js::import = "None"]
pub fn get_sdp_mid(this: &RtcIceCandidateInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sdpMid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
#[js::import = "None = $1"]
pub fn set_sdp_mid(this: &RtcIceCandidateInit, val: Option<&str>) { panic!("web-sys intrinsic") }
impl RtcIceCandidateInit {
    #[doc = "Construct a new `RtcIceCandidateInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidateInit`*"]
#[js::import = "None"]
pub fn new(candidate: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_candidate(candidate);
        ret
    }
    #[deprecated = "Use `set_candidate()` instead."]
#[js::import = "None"]
pub fn candidate(&mut self, val: &str) -> &mut Self { self.set_candidate(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_sdp_m_line_index()` instead."]
#[js::import = "None"]
pub fn sdp_m_line_index(&mut self, val: Option<u16>) -> &mut Self { self.set_sdp_m_line_index(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_sdp_mid()` instead."]
#[js::import = "None"]
pub fn sdp_mid(&mut self, val: Option<&str>) -> &mut Self { self.set_sdp_mid(val) { panic!("web-sys intrinsic") }
        self
    }
}
