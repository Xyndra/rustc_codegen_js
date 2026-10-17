#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeColorMatrixElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgfeColorMatrixElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeColorMatrixElement`*"]
impl SvgfeColorMatrixElement {
    #[js::import = "$0.in1"]
pub fn in1(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeColorMatrixElement`*"]
impl SvgfeColorMatrixElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumberList")]
    #[doc = "Getter for the `values` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgfeColorMatrixElement`*"]
impl SvgfeColorMatrixElement {
    #[js::import = "$0.values"]
pub fn values(&self) -> SvgAnimatedNumberList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
impl SvgfeColorMatrixElement {
    #[js::import = "$0.x"]
pub fn x(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
impl SvgfeColorMatrixElement {
    #[js::import = "$0.y"]
pub fn y(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
impl SvgfeColorMatrixElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeColorMatrixElement`*"]
impl SvgfeColorMatrixElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEColorMatrixElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeColorMatrixElement`*"]
impl SvgfeColorMatrixElement {
    #[js::import = "$0.result"]
pub fn result(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
impl SvgfeColorMatrixElement {
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_MATRIX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_MATRIX: u16 = 1u64 as u16;
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_SATURATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_SATURATE: u16 = 2u64 as u16;
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_HUEROTATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_HUEROTATE: u16 = 3u64 as u16;
    #[doc = "The `SVGFEColorMatrixElement.SVG_FECOLORMATRIX_TYPE_LUMINANCETOALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeColorMatrixElement`*"]
    pub const SVG_FECOLORMATRIX_TYPE_LUMINANCETOALPHA: u16 = 4u64 as u16;
}
