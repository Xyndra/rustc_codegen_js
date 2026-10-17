#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TrackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TrackEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &TrackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &TrackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &TrackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &TrackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &TrackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &TrackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
#[js::import = "None"]
pub fn get_track(this: &TrackEventInit) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `track` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_track(this: &TrackEventInit, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
impl TrackEventInit {
    #[doc = "Construct a new `TrackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEventInit`*"]
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
    #[deprecated = "Use `set_track()` instead."]
#[js::import = "None"]
pub fn track(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_track(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for TrackEventInit {
    fn default() -> Self {
        Self::new()
    }
}
