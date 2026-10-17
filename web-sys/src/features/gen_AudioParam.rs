#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioParam` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioParam(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.value"]
pub fn value(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `defaultValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/defaultValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.defaultValue"]
pub fn default_value(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `minValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/minValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.minValue"]
pub fn min_value(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/maxValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.maxValue"]
pub fn max_value(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancelScheduledValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/cancelScheduledValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.cancelScheduledValues($1)"]
pub fn cancel_scheduled_values(&self, start_time: f64) -> Result<AudioParam, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `exponentialRampToValueAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/exponentialRampToValueAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.exponentialRampToValueAtTime($1, $2)"]
pub fn exponential_ramp_to_value_at_time(&self, value: f32, end_time: f64) -> Result<AudioParam, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `linearRampToValueAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/linearRampToValueAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.linearRampToValueAtTime($1, $2)"]
pub fn linear_ramp_to_value_at_time(&self, value: f32, end_time: f64) -> Result<AudioParam, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTargetAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setTargetAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.setTargetAtTime($1, $2, $3)"]
pub fn set_target_at_time(&self, target: f32, start_time: f64, time_constant: f64) -> Result<AudioParam, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setValueAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.setValueAtTime($1, $2)"]
pub fn set_value_at_time(&self, value: f32, start_time: f64) -> Result<AudioParam, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setValueCurveAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueCurveAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.setValueCurveAtTime($1, $2, $3)"]
pub fn set_value_curve_at_time(&self, values: &mut [f32], start_time: f64, duration: f64) -> Result<AudioParam, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setValueCurveAtTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueCurveAtTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`*"]
impl AudioParam {
    #[js::import = "$0.setValueCurveAtTime($1, $2, $3)"]
pub fn set_value_curve_at_time_with_f32_array(&self, values: &::js_sys::Float32Array, start_time: f64, duration: f64) -> Result<AudioParam, JsValue> { panic!("web-sys intrinsic") }
}
