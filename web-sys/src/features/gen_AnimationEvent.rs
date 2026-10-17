#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnimationEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/animationName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
impl AnimationEvent {
    #[js::import = "$0.animationName"]
pub fn animation_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/elapsedTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
impl AnimationEvent {
    #[js::import = "$0.elapsedTime"]
pub fn elapsed_time(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/pseudoElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
impl AnimationEvent {
    #[js::import = "$0.pseudoElement"]
pub fn pseudo_element(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`*"]
impl AnimationEvent {
    #[js::import = "new AnimationEvent($0)"]
pub fn new(type_: &str) -> Result<AnimationEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AnimationEventInit")]
    #[doc = "The `new AnimationEvent(..)` constructor, creating a new instance of `AnimationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent/AnimationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEvent`, `AnimationEventInit`*"]
impl AnimationEvent {
    #[js::import = "new AnimationEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &AnimationEventInit,) -> Result<AnimationEvent, JsValue> { panic!("web-sys intrinsic") }
}
