#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgGeometryElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgGeometryElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `pathLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/pathLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgGeometryElement`*"]
impl SvgGeometryElement {
    #[js::import = "$0.pathLength"]
pub fn path_length(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPoint")]
    #[doc = "The `getPointAtLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getPointAtLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`, `SvgPoint`*"]
impl SvgGeometryElement {
    #[js::import = "$0.getPointAtLength($1)"]
pub fn get_point_at_length(&self, distance: f32) -> Result<SvgPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getTotalLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/getTotalLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
impl SvgGeometryElement {
    #[js::import = "$0.getTotalLength()"]
pub fn get_total_length(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `isPointInFill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInFill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
impl SvgGeometryElement {
    #[js::import = "$0.isPointInFill()"]
pub fn is_point_in_fill(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `isPointInFill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInFill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `SvgGeometryElement`*"]
impl SvgGeometryElement {
    #[js::import = "$0.isPointInFill($1)"]
pub fn is_point_in_fill_with_point(&self, point: &DomPointInit) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGeometryElement`*"]
impl SvgGeometryElement {
    #[js::import = "$0.isPointInStroke()"]
pub fn is_point_in_stroke(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGeometryElement/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `SvgGeometryElement`*"]
impl SvgGeometryElement {
    #[js::import = "$0.isPointInStroke($1)"]
pub fn is_point_in_stroke_with_point(&self, point: &DomPointInit) -> bool { panic!("web-sys intrinsic") }
}
