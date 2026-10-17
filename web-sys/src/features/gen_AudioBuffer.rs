#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioBuffer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioBuffer(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.sampleRate"]
pub fn sample_rate(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.duration"]
pub fn duration(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/numberOfChannels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.numberOfChannels"]
pub fn number_of_channels(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioBufferOptions")]
    #[doc = "The `new AudioBuffer(..)` constructor, creating a new instance of `AudioBuffer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/AudioBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `AudioBufferOptions`*"]
impl AudioBuffer {
    #[js::import = "new AudioBuffer($0)"]
pub fn new(options: &AudioBufferOptions) -> Result<AudioBuffer, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyFromChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.copyFromChannel($1, $2)"]
pub fn copy_from_channel(&self, destination: &mut [f32], channel_number: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyFromChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.copyFromChannel($1, $2)"]
pub fn copy_from_channel_with_f32_array(&self, destination: &::js_sys::Float32Array, channel_number: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyFromChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.copyFromChannel($1, $2, $3)"]
pub fn copy_from_channel_with_start_in_channel(&self, destination: &mut [f32], channel_number: i32, start_in_channel: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyFromChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.copyFromChannel($1, $2, $3)"]
pub fn copy_from_channel_with_f32_array_and_start_in_channel(&self, destination: &::js_sys::Float32Array, channel_number: i32, start_in_channel: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyToChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.copyToChannel($1, $2)"]
pub fn copy_to_channel(&self, source: &[f32], channel_number: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyToChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.copyToChannel($1, $2)"]
pub fn copy_to_channel_with_f32_array(&self, source: &::js_sys::Float32Array, channel_number: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyToChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.copyToChannel($1, $2, $3)"]
pub fn copy_to_channel_with_start_in_channel(&self, source: &[f32], channel_number: i32, start_in_channel: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyToChannel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.copyToChannel($1, $2, $3)"]
pub fn copy_to_channel_with_f32_array_and_start_in_channel(&self, source: &::js_sys::Float32Array, channel_number: i32, start_in_channel: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getChannelData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/getChannelData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`*"]
impl AudioBuffer {
    #[js::import = "$0.getChannelData($1)"]
pub fn get_channel_data(&self, channel: u32) -> Result<::alloc::vec::Vec<f32>, JsValue> { panic!("web-sys intrinsic") }
}
