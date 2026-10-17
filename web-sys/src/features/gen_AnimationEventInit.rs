#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnimationEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &AnimationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &AnimationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &AnimationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &AnimationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &AnimationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &AnimationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None"]
pub fn get_animation_name(this: &AnimationEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_animation_name(this: &AnimationEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None"]
pub fn get_elapsed_time(this: &AnimationEventInit) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_elapsed_time(this: &AnimationEventInit, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None"]
pub fn get_pseudo_element(this: &AnimationEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_pseudo_element(this: &AnimationEventInit, val: &str) { panic!("web-sys intrinsic") }
impl AnimationEventInit {
    #[doc = "Construct a new `AnimationEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEventInit`*"]
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
    #[deprecated = "Use `set_animation_name()` instead."]
#[js::import = "None"]
pub fn animation_name(&mut self, val: &str) -> &mut Self { self.set_animation_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_elapsed_time()` instead."]
#[js::import = "None"]
pub fn elapsed_time(&mut self, val: f32) -> &mut Self { self.set_elapsed_time(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_pseudo_element()` instead."]
#[js::import = "None"]
pub fn pseudo_element(&mut self, val: &str) -> &mut Self { self.set_pseudo_element(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AnimationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
