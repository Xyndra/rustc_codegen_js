#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "Getter for the `context` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/context)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `BaseAudioContext`*"]
impl AudioNode {
    #[js::import = "$0.context"]
pub fn context(&self) -> BaseAudioContext { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `numberOfInputs` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/numberOfInputs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.numberOfInputs"]
pub fn number_of_inputs(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `numberOfOutputs` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/numberOfOutputs)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.numberOfOutputs"]
pub fn number_of_outputs(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.channelCount"]
pub fn channel_count(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.channelCount = $1"]
pub fn set_channel_count(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Getter for the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCountMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `ChannelCountMode`*"]
impl AudioNode {
    #[js::import = "$0.channelCountMode"]
pub fn channel_count_mode(&self) -> ChannelCountMode { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Setter for the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCountMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `ChannelCountMode`*"]
impl AudioNode {
    #[js::import = "$0.channelCountMode = $1"]
pub fn set_channel_count_mode(&self, value: ChannelCountMode) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Getter for the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelInterpretation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `ChannelInterpretation`*"]
impl AudioNode {
    #[js::import = "$0.channelInterpretation"]
pub fn channel_interpretation(&self) -> ChannelInterpretation { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Setter for the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelInterpretation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `ChannelInterpretation`*"]
impl AudioNode {
    #[js::import = "$0.channelInterpretation = $1"]
pub fn set_channel_interpretation(&self, value: ChannelInterpretation) { panic!("web-sys intrinsic") }
}
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.connect($1)"]
pub fn connect_with_audio_node(&self, destination: &AudioNode) -> Result<AudioNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.connect($1, $2)"]
pub fn connect_with_audio_node_and_output(&self, destination: &AudioNode, output: u32) -> Result<AudioNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.connect($1, $2, $3)"]
pub fn connect_with_audio_node_and_output_and_input(&self, destination: &AudioNode, output: u32, input: u32) -> Result<AudioNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
impl AudioNode {
    #[js::import = "$0.connect($1)"]
pub fn connect_with_audio_param(&self, destination: &AudioParam) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "The `connect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
impl AudioNode {
    #[js::import = "$0.connect($1, $2)"]
pub fn connect_with_audio_param_and_output(&self, destination: &AudioParam, output: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.disconnect()"]
pub fn disconnect(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.disconnect($1)"]
pub fn disconnect_with_output(&self, output: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.disconnect($1)"]
pub fn disconnect_with_audio_node(&self, destination: &AudioNode) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.disconnect($1, $2)"]
pub fn disconnect_with_audio_node_and_output(&self, destination: &AudioNode, output: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`*"]
impl AudioNode {
    #[js::import = "$0.disconnect($1, $2, $3)"]
pub fn disconnect_with_audio_node_and_output_and_input(&self, destination: &AudioNode, output: u32, input: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
impl AudioNode {
    #[js::import = "$0.disconnect($1)"]
pub fn disconnect_with_audio_param(&self, destination: &AudioParam) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioNode`, `AudioParam`*"]
impl AudioNode {
    #[js::import = "$0.disconnect($1, $2)"]
pub fn disconnect_with_audio_param_and_output(&self, destination: &AudioParam, output: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
