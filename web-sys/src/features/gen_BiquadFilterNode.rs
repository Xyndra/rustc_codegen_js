#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BiquadFilterNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BiquadFilterNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*"]
impl BiquadFilterNode {
    #[js::import = "$0.type"]
pub fn type_(&self) -> BiquadFilterType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BiquadFilterType")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`, `BiquadFilterType`*"]
impl BiquadFilterNode {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: BiquadFilterType) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/frequency)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.frequency"]
pub fn frequency(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `detune` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/detune)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.detune"]
pub fn detune(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `Q` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/Q)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.Q"]
pub fn q(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `gain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/gain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.gain"]
pub fn gain(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "new BiquadFilterNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<BiquadFilterNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "BiquadFilterOptions",))]
    #[doc = "The `new BiquadFilterNode(..)` constructor, creating a new instance of `BiquadFilterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/BiquadFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `BiquadFilterNode`, `BiquadFilterOptions`*"]
impl BiquadFilterNode {
    #[js::import = "new BiquadFilterNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &BiquadFilterOptions,) -> Result<BiquadFilterNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response(&self, frequency_hz: &mut [f32], mag_response: &mut [f32], phase_response: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_array_and_f32_slice_and_f32_slice(&self, frequency_hz: &::js_sys::Float32Array, mag_response: &mut [f32], phase_response: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_slice_and_f32_array_and_f32_slice(&self, frequency_hz: &mut [f32], mag_response: &::js_sys::Float32Array, phase_response: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_array_and_f32_array_and_f32_slice(&self, frequency_hz: &::js_sys::Float32Array, mag_response: &::js_sys::Float32Array, phase_response: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_slice_and_f32_slice_and_f32_array(&self, frequency_hz: &mut [f32], mag_response: &mut [f32], phase_response: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_array_and_f32_slice_and_f32_array(&self, frequency_hz: &::js_sys::Float32Array, mag_response: &mut [f32], phase_response: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_slice_and_f32_array_and_f32_array(&self, frequency_hz: &mut [f32], mag_response: &::js_sys::Float32Array, phase_response: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BiquadFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BiquadFilterNode`*"]
impl BiquadFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_array_and_f32_array_and_f32_array(&self, frequency_hz: &::js_sys::Float32Array, mag_response: &::js_sys::Float32Array, phase_response: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
