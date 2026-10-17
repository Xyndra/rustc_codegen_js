#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPlaybackEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnimationPlaybackEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &AnimationPlaybackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &AnimationPlaybackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &AnimationPlaybackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &AnimationPlaybackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &AnimationPlaybackEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &AnimationPlaybackEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None"]
pub fn get_current_time(this: &AnimationPlaybackEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_current_time(this: &AnimationPlaybackEventInit, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timelineTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None"]
pub fn get_timeline_time(this: &AnimationPlaybackEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timelineTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
#[js::import = "None = $1"]
pub fn set_timeline_time(this: &AnimationPlaybackEventInit, val: Option<f64>) { panic!("web-sys intrinsic") }
impl AnimationPlaybackEventInit {
    #[doc = "Construct a new `AnimationPlaybackEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEventInit`*"]
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
    #[deprecated = "Use `set_current_time()` instead."]
#[js::import = "None"]
pub fn current_time(&mut self, val: Option<f64>) -> &mut Self { self.set_current_time(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_timeline_time()` instead."]
#[js::import = "None"]
pub fn timeline_time(&mut self, val: Option<f64>) -> &mut Self { self.set_timeline_time(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AnimationPlaybackEventInit {
    fn default() -> Self {
        Self::new()
    }
}
