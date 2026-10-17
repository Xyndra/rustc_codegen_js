#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnalyserNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnalyserNode(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.fftSize"]
pub fn fft_size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `fftSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/fftSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.fftSize = $1"]
pub fn set_fft_size(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `frequencyBinCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/frequencyBinCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.frequencyBinCount"]
pub fn frequency_bin_count(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.minDecibels"]
pub fn min_decibels(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `minDecibels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/minDecibels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.minDecibels = $1"]
pub fn set_min_decibels(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.maxDecibels"]
pub fn max_decibels(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `maxDecibels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/maxDecibels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.maxDecibels = $1"]
pub fn set_max_decibels(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.smoothingTimeConstant"]
pub fn smoothing_time_constant(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `smoothingTimeConstant` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/smoothingTimeConstant)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.smoothingTimeConstant = $1"]
pub fn set_smoothing_time_constant(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new AnalyserNode(..)` constructor, creating a new instance of `AnalyserNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/AnalyserNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`, `BaseAudioContext`*"]
impl AnalyserNode {
    #[js::import = "new AnalyserNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<AnalyserNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "AnalyserOptions", feature = "BaseAudioContext",))]
    #[doc = "The `new AnalyserNode(..)` constructor, creating a new instance of `AnalyserNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/AnalyserNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`, `AnalyserOptions`, `BaseAudioContext`*"]
impl AnalyserNode {
    #[js::import = "new AnalyserNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &AnalyserOptions,) -> Result<AnalyserNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getByteFrequencyData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteFrequencyData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.getByteFrequencyData($1)"]
pub fn get_byte_frequency_data(&self, array: &mut [u8]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getByteFrequencyData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteFrequencyData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.getByteFrequencyData($1)"]
pub fn get_byte_frequency_data_with_u8_array(&self, array: &::js_sys::Uint8Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getByteTimeDomainData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteTimeDomainData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.getByteTimeDomainData($1)"]
pub fn get_byte_time_domain_data(&self, array: &mut [u8]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getByteTimeDomainData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getByteTimeDomainData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.getByteTimeDomainData($1)"]
pub fn get_byte_time_domain_data_with_u8_array(&self, array: &::js_sys::Uint8Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFloatFrequencyData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatFrequencyData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.getFloatFrequencyData($1)"]
pub fn get_float_frequency_data(&self, array: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFloatFrequencyData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatFrequencyData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.getFloatFrequencyData($1)"]
pub fn get_float_frequency_data_with_f32_array(&self, array: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFloatTimeDomainData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatTimeDomainData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.getFloatTimeDomainData($1)"]
pub fn get_float_time_domain_data(&self, array: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFloatTimeDomainData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnalyserNode/getFloatTimeDomainData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnalyserNode`*"]
impl AnalyserNode {
    #[js::import = "$0.getFloatTimeDomainData($1)"]
pub fn get_float_time_domain_data_with_f32_array(&self, array: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
