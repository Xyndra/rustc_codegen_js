#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgNumber` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgNumber(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`*"]
impl SvgNumber {
    #[js::import = "$0.value"]
pub fn value(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumber/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`*"]
impl SvgNumber {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: f32) { panic!("web-sys intrinsic") }
}
