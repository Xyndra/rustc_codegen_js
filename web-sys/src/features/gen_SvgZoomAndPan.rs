#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgZoomAndPan` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgZoomAndPan(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
impl SvgZoomAndPan {
    #[js::import = "$0.zoomAndPan"]
pub fn zoom_and_pan(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGZoomAndPan/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
impl SvgZoomAndPan {
    #[js::import = "$0.zoomAndPan = $1"]
pub fn set_zoom_and_pan(&self, value: u16) { panic!("web-sys intrinsic") }
}
impl SvgZoomAndPan {
    #[doc = "The `SVGZoomAndPan.SVG_ZOOMANDPAN_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGZoomAndPan.SVG_ZOOMANDPAN_DISABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
    #[doc = "The `SVGZoomAndPan.SVG_ZOOMANDPAN_MAGNIFY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgZoomAndPan`*"]
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
