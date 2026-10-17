#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcPeerConnectionIceEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcPeerConnectionIceEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &RtcPeerConnectionIceEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &RtcPeerConnectionIceEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &RtcPeerConnectionIceEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &RtcPeerConnectionIceEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &RtcPeerConnectionIceEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &RtcPeerConnectionIceEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcIceCandidate")]
    #[doc = "Get the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnectionIceEventInit`*"]
#[js::import = "None"]
pub fn get_candidate(this: &RtcPeerConnectionIceEventInit) -> Option<RtcIceCandidate> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcIceCandidate")]
    #[doc = "Change the `candidate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCandidate`, `RtcPeerConnectionIceEventInit`*"]
#[js::import = "None = $1"]
pub fn set_candidate(this: &RtcPeerConnectionIceEventInit, val: Option<&RtcIceCandidate>) { panic!("web-sys intrinsic") }
impl RtcPeerConnectionIceEventInit {
    #[doc = "Construct a new `RtcPeerConnectionIceEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcPeerConnectionIceEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcIceCandidate")]
    #[deprecated = "Use `set_candidate()` instead."]
#[js::import = "None"]
pub fn candidate(&mut self, val: Option<&RtcIceCandidate>) -> &mut Self { self.set_candidate(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcPeerConnectionIceEventInit {
    fn default() -> Self {
        Self::new()
    }
}
