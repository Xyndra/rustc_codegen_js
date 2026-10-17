#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletProcessor` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletProcessor`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioWorkletProcessor(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MessagePort")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletProcessor`, `MessagePort`*"]
impl AudioWorkletProcessor {
    #[js::import = "$0.port"]
pub fn port(&self) -> Result<MessagePort, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new AudioWorkletProcessor(..)` constructor, creating a new instance of `AudioWorkletProcessor`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/AudioWorkletProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletProcessor`*"]
impl AudioWorkletProcessor {
    #[js::import = "new AudioWorkletProcessor()"]
pub fn new() -> Result<AudioWorkletProcessor, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioWorkletNodeOptions")]
    #[doc = "The `new AudioWorkletProcessor(..)` constructor, creating a new instance of `AudioWorkletProcessor`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor/AudioWorkletProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNodeOptions`, `AudioWorkletProcessor`*"]
impl AudioWorkletProcessor {
    #[js::import = "new AudioWorkletProcessor($0)"]
pub fn new_with_options(options: &AudioWorkletNodeOptions,) -> Result<AudioWorkletProcessor, JsValue> { panic!("web-sys intrinsic") }
}
