#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioWorkletGlobalScope(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `currentFrame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
impl AudioWorkletGlobalScope {
    #[js::import = "$0.currentFrame"]
pub fn current_frame(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `currentTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/currentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
impl AudioWorkletGlobalScope {
    #[js::import = "$0.currentTime"]
pub fn current_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
impl AudioWorkletGlobalScope {
    #[js::import = "$0.sampleRate"]
pub fn sample_rate(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `registerProcessor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletGlobalScope/registerProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletGlobalScope`*"]
impl AudioWorkletGlobalScope {
    #[js::import = "$0.registerProcessor($1, $2)"]
pub fn register_processor(&self, name: &str, processor_ctor: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
