#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgMaskElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMaskElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgMaskElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `maskUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMaskElement`*"]
impl SvgMaskElement {
    #[js::import = "$0.maskUnits"]
pub fn mask_units(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `maskContentUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/maskContentUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgMaskElement`*"]
impl SvgMaskElement {
    #[js::import = "$0.maskContentUnits"]
pub fn mask_content_units(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
impl SvgMaskElement {
    #[js::import = "$0.x"]
pub fn x(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
impl SvgMaskElement {
    #[js::import = "$0.y"]
pub fn y(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
impl SvgMaskElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGMaskElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgMaskElement`*"]
impl SvgMaskElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
impl SvgMaskElement {
    #[doc = "The `SVGMaskElement.SVG_MASKTYPE_LUMINANCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMaskElement`*"]
    pub const SVG_MASKTYPE_LUMINANCE: u16 = 0i64 as u16;
    #[doc = "The `SVGMaskElement.SVG_MASKTYPE_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMaskElement`*"]
    pub const SVG_MASKTYPE_ALPHA: u16 = 1u64 as u16;
}
