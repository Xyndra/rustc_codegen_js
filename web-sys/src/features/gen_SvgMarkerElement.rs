#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgMarkerElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgMarkerElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `refX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.refX"]
pub fn ref_x(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `refY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/refY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.refY"]
pub fn ref_y(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `markerUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.markerUnits"]
pub fn marker_units(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `markerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.markerWidth"]
pub fn marker_width(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `markerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/markerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.markerHeight"]
pub fn marker_height(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `orientType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.orientType"]
pub fn orient_type(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedAngle")]
    #[doc = "Getter for the `orientAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/orientAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedAngle`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.orientAngle"]
pub fn orient_angle(&self) -> SvgAnimatedAngle { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/viewBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.viewBox"]
pub fn view_box(&self) -> SvgAnimatedRect { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.preserveAspectRatio"]
pub fn preserve_aspect_ratio(&self) -> SvgAnimatedPreserveAspectRatio { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAngle")]
    #[doc = "The `setOrientToAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`, `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.setOrientToAngle($1)"]
pub fn set_orient_to_angle(&self, angle: &SvgAngle) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setOrientToAuto()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMarkerElement/setOrientToAuto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
impl SvgMarkerElement {
    #[js::import = "$0.setOrientToAuto()"]
pub fn set_orient_to_auto(&self) { panic!("web-sys intrinsic") }
}
impl SvgMarkerElement {
    #[doc = "The `SVGMarkerElement.SVG_MARKERUNITS_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKERUNITS_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKERUNITS_USERSPACEONUSE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKERUNITS_USERSPACEONUSE: u16 = 1u64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKERUNITS_STROKEWIDTH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKERUNITS_STROKEWIDTH: u16 = 2u64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKER_ORIENT_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKER_ORIENT_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKER_ORIENT_AUTO` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKER_ORIENT_AUTO: u16 = 1u64 as u16;
    #[doc = "The `SVGMarkerElement.SVG_MARKER_ORIENT_ANGLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMarkerElement`*"]
    pub const SVG_MARKER_ORIENT_ANGLE: u16 = 2u64 as u16;
}
