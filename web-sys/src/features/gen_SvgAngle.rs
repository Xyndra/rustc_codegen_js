#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAngle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAngle(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `unitType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/unitType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
impl SvgAngle {
    #[js::import = "$0.unitType"]
pub fn unit_type(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
impl SvgAngle {
    #[js::import = "$0.value"]
pub fn value(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
impl SvgAngle {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `valueInSpecifiedUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
impl SvgAngle {
    #[js::import = "$0.valueInSpecifiedUnits"]
pub fn value_in_specified_units(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `valueInSpecifiedUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
impl SvgAngle {
    #[js::import = "$0.valueInSpecifiedUnits = $1"]
pub fn set_value_in_specified_units(&self, value: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `valueAsString` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
impl SvgAngle {
    #[js::import = "$0.valueAsString"]
pub fn value_as_string(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `valueAsString` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
impl SvgAngle {
    #[js::import = "$0.valueAsString = $1"]
pub fn set_value_as_string(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `convertToSpecifiedUnits()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/convertToSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
impl SvgAngle {
    #[js::import = "$0.convertToSpecifiedUnits($1)"]
pub fn convert_to_specified_units(&self, unit_type: u16) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `newValueSpecifiedUnits()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/newValueSpecifiedUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
impl SvgAngle {
    #[js::import = "$0.newValueSpecifiedUnits($1, $2)"]
pub fn new_value_specified_units(&self, unit_type: u16, value_in_specified_units: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl SvgAngle {
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_UNSPECIFIED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_UNSPECIFIED: u16 = 1u64 as u16;
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_DEG` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_DEG: u16 = 2u64 as u16;
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_RAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_RAD: u16 = 3u64 as u16;
    #[doc = "The `SVGAngle.SVG_ANGLETYPE_GRAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`*"]
    pub const SVG_ANGLETYPE_GRAD: u16 = 4u64 as u16;
}
