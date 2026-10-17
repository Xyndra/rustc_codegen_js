#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgViewElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgViewElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/viewBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgViewElement`*"]
impl SvgViewElement {
    #[js::import = "$0.viewBox"]
pub fn view_box(&self) -> SvgAnimatedRect { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgViewElement`*"]
impl SvgViewElement {
    #[js::import = "$0.preserveAspectRatio"]
pub fn preserve_aspect_ratio(&self) -> SvgAnimatedPreserveAspectRatio { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
impl SvgViewElement {
    #[js::import = "$0.zoomAndPan"]
pub fn zoom_and_pan(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGViewElement/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
impl SvgViewElement {
    #[js::import = "$0.zoomAndPan = $1"]
pub fn set_zoom_and_pan(&self, value: u16) { panic!("web-sys intrinsic") }
}
impl SvgViewElement {
    #[doc = "The `SVGViewElement.SVG_ZOOMANDPAN_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGViewElement.SVG_ZOOMANDPAN_DISABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
    #[doc = "The `SVGViewElement.SVG_ZOOMANDPAN_MAGNIFY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgViewElement`*"]
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
