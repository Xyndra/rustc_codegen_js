#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamTrackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStreamTrackEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &MediaStreamTrackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &MediaStreamTrackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &MediaStreamTrackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &MediaStreamTrackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &MediaStreamTrackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &MediaStreamTrackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEventInit`*"]
#[js::import = "None"]
pub fn get_track(this: &MediaStreamTrackEventInit) -> MediaStreamTrack { panic!("web-sys intrinsic") }
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_track(this: &MediaStreamTrackEventInit, val: &MediaStreamTrack) { panic!("web-sys intrinsic") }
impl MediaStreamTrackEventInit {
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Construct a new `MediaStreamTrackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEventInit`*"]
#[js::import = "None"]
pub fn new(track: &MediaStreamTrack) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_track(track);
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
    #[cfg(feature = "MediaStreamTrack")]
    #[deprecated = "Use `set_track()` instead."]
#[js::import = "None"]
pub fn track(&mut self, val: &MediaStreamTrack) -> &mut Self { self.set_track(val) { panic!("web-sys intrinsic") }
        self
    }
}
