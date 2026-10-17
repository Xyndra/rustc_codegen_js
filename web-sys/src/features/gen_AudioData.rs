#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioData(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSampleFormat")]
    #[doc = "Getter for the `format` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/format)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioSampleFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.format"]
pub fn format(&self) -> Option<AudioSampleFormat> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/sampleRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.sampleRate"]
pub fn sample_rate(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `numberOfFrames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/numberOfFrames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.numberOfFrames"]
pub fn number_of_frames(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `numberOfChannels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/numberOfChannels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.numberOfChannels"]
pub fn number_of_channels(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.duration"]
pub fn duration(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.timestamp"]
pub fn timestamp(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataInit")]
    #[doc = "The `new AudioData(..)` constructor, creating a new instance of `AudioData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/AudioData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "new AudioData($0)"]
pub fn new(init: &AudioDataInit) -> Result<AudioData, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataCopyToOptions")]
    #[doc = "The `allocationSize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/allocationSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.allocationSize($1)"]
pub fn allocation_size(&self, options: &AudioDataCopyToOptions) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.clone()"]
pub fn clone(&self) -> Result<AudioData, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataCopyToOptions")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.copyTo($1, $2)"]
pub fn copy_to_with_buffer_source(&self, destination: &::js_sys::Object, options: &AudioDataCopyToOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataCopyToOptions")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.copyTo($1, $2)"]
pub fn copy_to_with_u8_slice(&self, destination: &mut [u8], options: &AudioDataCopyToOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDataCopyToOptions")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioData`, `AudioDataCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl AudioData {
    #[js::import = "$0.copyTo($1, $2)"]
pub fn copy_to_with_u8_array(&self, destination: &::js_sys::Uint8Array, options: &AudioDataCopyToOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
