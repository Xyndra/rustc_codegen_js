#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IirFilterNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IirFilterNode(pub ::wasm_bindgen::JsValue);
    #[cfg(all(feature = "BaseAudioContext", feature = "IirFilterOptions",))]
    #[doc = "The `new IirFilterNode(..)` constructor, creating a new instance of `IirFilterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/IIRFilterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `IirFilterNode`, `IirFilterOptions`*"]
impl IirFilterNode {
    #[js::import = "new IIRFilterNode($0, $1)"]
pub fn new(context: &BaseAudioContext, options: &IirFilterOptions,) -> Result<IirFilterNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
impl IirFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response(&self, frequency_hz: &mut [f32], mag_response: &mut [f32], phase_response: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
impl IirFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_array_and_f32_slice_and_f32_slice(&self, frequency_hz: &::js_sys::Float32Array, mag_response: &mut [f32], phase_response: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
impl IirFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_slice_and_f32_array_and_f32_slice(&self, frequency_hz: &mut [f32], mag_response: &::js_sys::Float32Array, phase_response: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
impl IirFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_array_and_f32_array_and_f32_slice(&self, frequency_hz: &::js_sys::Float32Array, mag_response: &::js_sys::Float32Array, phase_response: &mut [f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
impl IirFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_slice_and_f32_slice_and_f32_array(&self, frequency_hz: &mut [f32], mag_response: &mut [f32], phase_response: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
impl IirFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_array_and_f32_slice_and_f32_array(&self, frequency_hz: &::js_sys::Float32Array, mag_response: &mut [f32], phase_response: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
impl IirFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_slice_and_f32_array_and_f32_array(&self, frequency_hz: &mut [f32], mag_response: &::js_sys::Float32Array, phase_response: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFrequencyResponse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IIRFilterNode/getFrequencyResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IirFilterNode`*"]
impl IirFilterNode {
    #[js::import = "$0.getFrequencyResponse($1, $2, $3)"]
pub fn get_frequency_response_with_f32_array_and_f32_array_and_f32_array(&self, frequency_hz: &::js_sys::Float32Array, mag_response: &::js_sys::Float32Array, phase_response: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
