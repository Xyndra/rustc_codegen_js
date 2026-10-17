#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioProcessingEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioProcessingEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioProcessingEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `playbackTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/playbackTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioProcessingEvent`*"]
impl AudioProcessingEvent {
    #[js::import = "$0.playbackTime"]
pub fn playback_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `inputBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/inputBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*"]
impl AudioProcessingEvent {
    #[js::import = "$0.inputBuffer"]
pub fn input_buffer(&self) -> Result<AudioBuffer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `outputBuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioProcessingEvent/outputBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioProcessingEvent`*"]
impl AudioProcessingEvent {
    #[js::import = "$0.outputBuffer"]
pub fn output_buffer(&self) -> Result<AudioBuffer, JsValue> { panic!("web-sys intrinsic") }
}
