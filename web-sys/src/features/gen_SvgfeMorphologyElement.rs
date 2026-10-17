#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeMorphologyElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgfeMorphologyElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMorphologyElement`*"]
impl SvgfeMorphologyElement {
    #[js::import = "$0.in1"]
pub fn in1(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `operator` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/operator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeMorphologyElement`*"]
impl SvgfeMorphologyElement {
    #[js::import = "$0.operator"]
pub fn operator(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeMorphologyElement`*"]
impl SvgfeMorphologyElement {
    #[js::import = "$0.radiusX"]
pub fn radius_x(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/radiusY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeMorphologyElement`*"]
impl SvgfeMorphologyElement {
    #[js::import = "$0.radiusY"]
pub fn radius_y(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
impl SvgfeMorphologyElement {
    #[js::import = "$0.x"]
pub fn x(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
impl SvgfeMorphologyElement {
    #[js::import = "$0.y"]
pub fn y(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
impl SvgfeMorphologyElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeMorphologyElement`*"]
impl SvgfeMorphologyElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMorphologyElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMorphologyElement`*"]
impl SvgfeMorphologyElement {
    #[js::import = "$0.result"]
pub fn result(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
impl SvgfeMorphologyElement {
    #[doc = "The `SVGFEMorphologyElement.SVG_MORPHOLOGY_OPERATOR_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*"]
    pub const SVG_MORPHOLOGY_OPERATOR_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFEMorphologyElement.SVG_MORPHOLOGY_OPERATOR_ERODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*"]
    pub const SVG_MORPHOLOGY_OPERATOR_ERODE: u16 = 1u64 as u16;
    #[doc = "The `SVGFEMorphologyElement.SVG_MORPHOLOGY_OPERATOR_DILATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMorphologyElement`*"]
    pub const SVG_MORPHOLOGY_OPERATOR_DILATE: u16 = 2u64 as u16;
}
