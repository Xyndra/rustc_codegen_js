#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioWorkletNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioWorkletNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioParamMap")]
    #[doc = "Getter for the `parameters` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/parameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParamMap`, `AudioWorkletNode`*"]
impl AudioWorkletNode {
    #[js::import = "$0.parameters"]
pub fn parameters(&self) -> Result<AudioParamMap, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MessagePort")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`, `MessagePort`*"]
impl AudioWorkletNode {
    #[js::import = "$0.port"]
pub fn port(&self) -> Result<MessagePort, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onprocessorerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`*"]
impl AudioWorkletNode {
    #[js::import = "$0.onprocessorerror"]
pub fn onprocessorerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onprocessorerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`*"]
impl AudioWorkletNode {
    #[js::import = "$0.onprocessorerror = $1"]
pub fn set_onprocessorerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new AudioWorkletNode(..)` constructor, creating a new instance of `AudioWorkletNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`, `BaseAudioContext`*"]
impl AudioWorkletNode {
    #[js::import = "new AudioWorkletNode($0, $1)"]
pub fn new(context: &BaseAudioContext, name: &str) -> Result<AudioWorkletNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "AudioWorkletNodeOptions", feature = "BaseAudioContext",))]
    #[doc = "The `new AudioWorkletNode(..)` constructor, creating a new instance of `AudioWorkletNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/AudioWorkletNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioWorkletNode`, `AudioWorkletNodeOptions`, `BaseAudioContext`*"]
impl AudioWorkletNode {
    #[js::import = "new AudioWorkletNode($0, $1, $2)"]
pub fn new_with_options(context: &BaseAudioContext, name: &str, options: &AudioWorkletNodeOptions,) -> Result<AudioWorkletNode, JsValue> { panic!("web-sys intrinsic") }
}
