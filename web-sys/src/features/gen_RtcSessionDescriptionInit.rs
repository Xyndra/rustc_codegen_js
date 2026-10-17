#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcSessionDescriptionInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcSessionDescriptionInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `sdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
#[js::import = "None"]
pub fn get_sdp(this: &RtcSessionDescriptionInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sdp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSessionDescriptionInit`*"]
#[js::import = "None = $1"]
pub fn set_sdp(this: &RtcSessionDescriptionInit, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
#[js::import = "None"]
pub fn get_type(this: &RtcSessionDescriptionInit) -> RtcSdpType { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RtcSessionDescriptionInit, val: RtcSdpType) { panic!("web-sys intrinsic") }
impl RtcSessionDescriptionInit {
    #[cfg(feature = "RtcSdpType")]
    #[doc = "Construct a new `RtcSessionDescriptionInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcSdpType`, `RtcSessionDescriptionInit`*"]
#[js::import = "None"]
pub fn new(type_: RtcSdpType) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_sdp()` instead."]
#[js::import = "None"]
pub fn sdp(&mut self, val: &str) -> &mut Self { self.set_sdp(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcSdpType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: RtcSdpType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
