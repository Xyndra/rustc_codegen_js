#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPlaybackEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnimationPlaybackEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
impl AnimationPlaybackEvent {
    #[js::import = "$0.currentTime"]
pub fn current_time(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `timelineTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/timelineTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
impl AnimationPlaybackEvent {
    #[js::import = "$0.timelineTime"]
pub fn timeline_time(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new AnimationPlaybackEvent(..)` constructor, creating a new instance of `AnimationPlaybackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/AnimationPlaybackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`*"]
impl AnimationPlaybackEvent {
    #[js::import = "new AnimationPlaybackEvent($0)"]
pub fn new(type_: &str) -> Result<AnimationPlaybackEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AnimationPlaybackEventInit")]
    #[doc = "The `new AnimationPlaybackEvent(..)` constructor, creating a new instance of `AnimationPlaybackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationPlaybackEvent/AnimationPlaybackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPlaybackEvent`, `AnimationPlaybackEventInit`*"]
impl AnimationPlaybackEvent {
    #[js::import = "new AnimationPlaybackEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &AnimationPlaybackEventInit,) -> Result<AnimationPlaybackEvent, JsValue> { panic!("web-sys intrinsic") }
}
