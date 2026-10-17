#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPreserveAspectRatio` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgPreserveAspectRatio(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
impl SvgPreserveAspectRatio {
    #[js::import = "$0.align"]
pub fn align(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
impl SvgPreserveAspectRatio {
    #[js::import = "$0.align = $1"]
pub fn set_align(&self, value: u16) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `meetOrSlice` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/meetOrSlice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
impl SvgPreserveAspectRatio {
    #[js::import = "$0.meetOrSlice"]
pub fn meet_or_slice(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `meetOrSlice` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPreserveAspectRatio/meetOrSlice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
impl SvgPreserveAspectRatio {
    #[js::import = "$0.meetOrSlice = $1"]
pub fn set_meet_or_slice(&self, value: u16) { panic!("web-sys intrinsic") }
}
impl SvgPreserveAspectRatio {
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_NONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_NONE: u16 = 1u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMINYMIN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMINYMIN: u16 = 2u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMIDYMIN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMIDYMIN: u16 = 3u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMAXYMIN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMAXYMIN: u16 = 4u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMINYMID` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMINYMID: u16 = 5u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMIDYMID` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMIDYMID: u16 = 6u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMAXYMID` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMAXYMID: u16 = 7u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMINYMAX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMINYMAX: u16 = 8u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMIDYMAX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMIDYMAX: u16 = 9u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_PRESERVEASPECTRATIO_XMAXYMAX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_PRESERVEASPECTRATIO_XMAXYMAX: u16 = 10u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_MEETORSLICE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_MEETORSLICE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_MEETORSLICE_MEET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_MEETORSLICE_MEET: u16 = 1u64 as u16;
    #[doc = "The `SVGPreserveAspectRatio.SVG_MEETORSLICE_SLICE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPreserveAspectRatio`*"]
    pub const SVG_MEETORSLICE_SLICE: u16 = 2u64 as u16;
}
