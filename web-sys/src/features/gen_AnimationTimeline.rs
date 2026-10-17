#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationTimeline` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationTimeline`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnimationTimeline(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationTimeline/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationTimeline`*"]
impl AnimationTimeline {
    #[js::import = "$0.currentTime"]
pub fn current_time(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
