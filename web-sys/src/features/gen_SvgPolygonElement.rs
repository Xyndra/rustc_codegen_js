#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPolygonElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPolygonElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgPolygonElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgPointList")]
    #[doc = "Getter for the `points` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/points)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*"]
impl SvgPolygonElement {
    #[js::import = "$0.points"]
pub fn points(&self) -> SvgPointList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPointList")]
    #[doc = "Getter for the `animatedPoints` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPolygonElement/animatedPoints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPointList`, `SvgPolygonElement`*"]
impl SvgPolygonElement {
    #[js::import = "$0.animatedPoints"]
pub fn animated_points(&self) -> SvgPointList { panic!("web-sys intrinsic") }
}
