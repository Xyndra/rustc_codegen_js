#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcTrackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcTrackEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &RtcTrackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &RtcTrackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &RtcTrackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &RtcTrackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &RtcTrackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &RtcTrackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Get the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEventInit`*"]
#[js::import = "None"]
pub fn get_receiver(this: &RtcTrackEventInit) -> RtcRtpReceiver { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtpReceiver")]
    #[doc = "Change the `receiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpReceiver`, `RtcTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_receiver(this: &RtcTrackEventInit, val: &RtcRtpReceiver) { panic!("web-sys intrinsic") }
    #[doc = "Get the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
#[js::import = "None"]
pub fn get_streams(this: &RtcTrackEventInit) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_streams(this: &RtcTrackEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEventInit`*"]
#[js::import = "None"]
pub fn get_track(this: &RtcTrackEventInit) -> MediaStreamTrack { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_track(this: &RtcTrackEventInit, val: &MediaStreamTrack) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "Get the `transceiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
#[js::import = "None"]
pub fn get_transceiver(this: &RtcTrackEventInit) -> RtcRtpTransceiver { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcRtpTransceiver")]
    #[doc = "Change the `transceiver` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_transceiver(this: &RtcTrackEventInit, val: &RtcRtpTransceiver) { panic!("web-sys intrinsic") }
impl RtcTrackEventInit {
    #[cfg(all(
        feature = "MediaStreamTrack",
        feature = "RtcRtpReceiver",
        feature = "RtcRtpTransceiver",
    ))]
    #[doc = "Construct a new `RtcTrackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `RtcRtpReceiver`, `RtcRtpTransceiver`, `RtcTrackEventInit`*"]
#[js::import = "None"]
pub fn new(receiver: &RtcRtpReceiver, track: &MediaStreamTrack, transceiver: &RtcRtpTransceiver,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_receiver(receiver);
        ret.set_track(track);
        ret.set_transceiver(transceiver);
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
    #[cfg(feature = "RtcRtpReceiver")]
    #[deprecated = "Use `set_receiver()` instead."]
#[js::import = "None"]
pub fn receiver(&mut self, val: &RtcRtpReceiver) -> &mut Self { self.set_receiver(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_streams()` instead."]
#[js::import = "None"]
pub fn streams(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_streams(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "MediaStreamTrack")]
    #[deprecated = "Use `set_track()` instead."]
#[js::import = "None"]
pub fn track(&mut self, val: &MediaStreamTrack) -> &mut Self { self.set_track(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcRtpTransceiver")]
    #[deprecated = "Use `set_transceiver()` instead."]
#[js::import = "None"]
pub fn transceiver(&mut self, val: &RtcRtpTransceiver) -> &mut Self { self.set_transceiver(val) { panic!("web-sys intrinsic") }
        self
    }
}
