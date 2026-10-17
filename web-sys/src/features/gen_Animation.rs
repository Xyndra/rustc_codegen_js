#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Animation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Animation(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.id = $1"]
pub fn set_id(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AnimationEffect")]
    #[doc = "Getter for the `effect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*"]
impl Animation {
    #[js::import = "$0.effect"]
pub fn effect(&self) -> Option<AnimationEffect> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AnimationEffect")]
    #[doc = "Setter for the `effect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/effect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*"]
impl Animation {
    #[js::import = "$0.effect = $1"]
pub fn set_effect(&self, value: Option<&AnimationEffect>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AnimationTimeline")]
    #[doc = "Getter for the `timeline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `AnimationTimeline`*"]
impl Animation {
    #[js::import = "$0.timeline"]
pub fn timeline(&self) -> Option<AnimationTimeline> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AnimationTimeline")]
    #[doc = "Setter for the `timeline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/timeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `AnimationTimeline`*"]
impl Animation {
    #[js::import = "$0.timeline = $1"]
pub fn set_timeline(&self, value: Option<&AnimationTimeline>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.startTime"]
pub fn start_time(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/startTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.startTime = $1"]
pub fn set_start_time(&self, value: Option<f64>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.currentTime"]
pub fn current_time(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.currentTime = $1"]
pub fn set_current_time(&self, value: Option<f64>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.playbackRate"]
pub fn playback_rate(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playbackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.playbackRate = $1"]
pub fn set_playback_rate(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AnimationPlayState")]
    #[doc = "Getter for the `playState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/playState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `AnimationPlayState`*"]
impl Animation {
    #[js::import = "$0.playState"]
pub fn play_state(&self) -> AnimationPlayState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pending` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pending)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.pending"]
pub fn pending(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.ready"]
pub fn ready(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `finished` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finished)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.finished"]
pub fn finished(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onfinish` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.onfinish"]
pub fn onfinish(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onfinish` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/onfinish)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.onfinish = $1"]
pub fn set_onfinish(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.oncancel"]
pub fn oncancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/oncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.oncancel = $1"]
pub fn set_oncancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Animation(..)` constructor, creating a new instance of `Animation`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "new Animation()"]
pub fn new() -> Result<Animation, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AnimationEffect")]
    #[doc = "The `new Animation(..)` constructor, creating a new instance of `Animation`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`*"]
impl Animation {
    #[js::import = "new Animation($0)"]
pub fn new_with_effect(effect: Option<&AnimationEffect>) -> Result<Animation, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "AnimationEffect", feature = "AnimationTimeline",))]
    #[doc = "The `new Animation(..)` constructor, creating a new instance of `Animation`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/Animation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`, `AnimationEffect`, `AnimationTimeline`*"]
impl Animation {
    #[js::import = "new Animation($0, $1)"]
pub fn new_with_effect_and_timeline(effect: Option<&AnimationEffect>, timeline: Option<&AnimationTimeline>,) -> Result<Animation, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/cancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.cancel()"]
pub fn cancel(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `finish()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/finish)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.finish()"]
pub fn finish(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `pause()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/pause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.pause()"]
pub fn pause(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `play()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/play)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.play()"]
pub fn play(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `reverse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/reverse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.reverse()"]
pub fn reverse(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `updatePlaybackRate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Animation/updatePlaybackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Animation`*"]
impl Animation {
    #[js::import = "$0.updatePlaybackRate($1)"]
pub fn update_playback_rate(&self, playback_rate: f64) { panic!("web-sys intrinsic") }
}
