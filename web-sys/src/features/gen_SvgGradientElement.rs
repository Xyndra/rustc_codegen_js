#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgGradientElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgGradientElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `gradientUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgGradientElement`*"]
impl SvgGradientElement {
    #[js::import = "$0.gradientUnits"]
pub fn gradient_units(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[doc = "Getter for the `gradientTransform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/gradientTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgGradientElement`*"]
impl SvgGradientElement {
    #[js::import = "$0.gradientTransform"]
pub fn gradient_transform(&self) -> SvgAnimatedTransformList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `spreadMethod` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/spreadMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgGradientElement`*"]
impl SvgGradientElement {
    #[js::import = "$0.spreadMethod"]
pub fn spread_method(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGradientElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgGradientElement`*"]
impl SvgGradientElement {
    #[js::import = "$0.href"]
pub fn href(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
impl SvgGradientElement {
    #[doc = "The `SVGGradientElement.SVG_SPREADMETHOD_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub const SVG_SPREADMETHOD_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGGradientElement.SVG_SPREADMETHOD_PAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub const SVG_SPREADMETHOD_PAD: u16 = 1u64 as u16;
    #[doc = "The `SVGGradientElement.SVG_SPREADMETHOD_REFLECT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub const SVG_SPREADMETHOD_REFLECT: u16 = 2u64 as u16;
    #[doc = "The `SVGGradientElement.SVG_SPREADMETHOD_REPEAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGradientElement`*"]
    pub const SVG_SPREADMETHOD_REPEAT: u16 = 3u64 as u16;
}
