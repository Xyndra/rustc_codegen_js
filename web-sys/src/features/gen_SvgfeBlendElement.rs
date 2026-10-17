#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeBlendElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgfeBlendElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeBlendElement`*"]
impl SvgfeBlendElement {
    #[js::import = "$0.in1"]
pub fn in1(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/in2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeBlendElement`*"]
impl SvgfeBlendElement {
    #[js::import = "$0.in2"]
pub fn in2(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgfeBlendElement`*"]
impl SvgfeBlendElement {
    #[js::import = "$0.mode"]
pub fn mode(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
impl SvgfeBlendElement {
    #[js::import = "$0.x"]
pub fn x(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
impl SvgfeBlendElement {
    #[js::import = "$0.y"]
pub fn y(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
impl SvgfeBlendElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgfeBlendElement`*"]
impl SvgfeBlendElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEBlendElement/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeBlendElement`*"]
impl SvgfeBlendElement {
    #[js::import = "$0.result"]
pub fn result(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
impl SvgfeBlendElement {
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_NORMAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_NORMAL: u16 = 1u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_MULTIPLY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_MULTIPLY: u16 = 2u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_SCREEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_SCREEN: u16 = 3u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_DARKEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_DARKEN: u16 = 4u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_LIGHTEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_LIGHTEN: u16 = 5u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_OVERLAY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_OVERLAY: u16 = 6u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_COLOR_DODGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_COLOR_DODGE: u16 = 7u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_COLOR_BURN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_COLOR_BURN: u16 = 8u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_HARD_LIGHT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_HARD_LIGHT: u16 = 9u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_SOFT_LIGHT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_SOFT_LIGHT: u16 = 10u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_DIFFERENCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_DIFFERENCE: u16 = 11u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_EXCLUSION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_EXCLUSION: u16 = 12u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_HUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_HUE: u16 = 13u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_SATURATION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_SATURATION: u16 = 14u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_COLOR: u16 = 15u64 as u16;
    #[doc = "The `SVGFEBlendElement.SVG_FEBLEND_MODE_LUMINOSITY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeBlendElement`*"]
    pub const SVG_FEBLEND_MODE_LUMINOSITY: u16 = 16u64 as u16;
}
