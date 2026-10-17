#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBufferSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioBufferSourceNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.buffer"]
pub fn buffer(&self) -> Option<AudioBuffer> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Setter for the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/buffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.buffer = $1"]
pub fn set_buffer(&self, value: Option<&AudioBuffer>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `playbackRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/playbackRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioParam`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.playbackRate"]
pub fn playback_rate(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `detune` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/detune)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioParam`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.detune"]
pub fn detune(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `loop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.loop"]
pub fn loop_(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `loop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.loop = $1"]
pub fn set_loop(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.loopStart"]
pub fn loop_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `loopStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.loopStart = $1"]
pub fn set_loop_start(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.loopEnd"]
pub fn loop_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `loopEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/loopEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.loopEnd = $1"]
pub fn set_loop_end(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[deprecated]
impl AudioBufferSourceNode {
    #[js::import = "$0.onended"]
pub fn onended(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[deprecated]
impl AudioBufferSourceNode {
    #[js::import = "$0.onended = $1"]
pub fn set_onended(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new AudioBufferSourceNode(..)` constructor, creating a new instance of `AudioBufferSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/AudioBufferSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `BaseAudioContext`*"]
impl AudioBufferSourceNode {
    #[js::import = "new AudioBufferSourceNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<AudioBufferSourceNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "AudioBufferSourceOptions", feature = "BaseAudioContext",))]
    #[doc = "The `new AudioBufferSourceNode(..)` constructor, creating a new instance of `AudioBufferSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/AudioBufferSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`, `AudioBufferSourceOptions`, `BaseAudioContext`*"]
impl AudioBufferSourceNode {
    #[js::import = "new AudioBufferSourceNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &AudioBufferSourceOptions,) -> Result<AudioBufferSourceNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.start()"]
pub fn start(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.start($1)"]
pub fn start_with_when(&self, when: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.start($1, $2)"]
pub fn start_with_when_and_grain_offset(&self, when: f64, offset: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
impl AudioBufferSourceNode {
    #[js::import = "$0.start($1, $2, $3)"]
pub fn start_with_when_and_grain_offset_and_grain_duration(&self, when: f64, offset: f64, duration: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[deprecated]
impl AudioBufferSourceNode {
    #[js::import = "$0.stop()"]
pub fn stop(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBufferSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBufferSourceNode`*"]
    #[deprecated]
impl AudioBufferSourceNode {
    #[js::import = "$0.stop($1)"]
pub fn stop_with_when(&self, when: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
