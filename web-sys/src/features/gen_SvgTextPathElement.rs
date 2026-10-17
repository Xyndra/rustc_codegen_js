#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTextPathElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextPathElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgTextPathElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `startOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/startOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgTextPathElement`*"]
impl SvgTextPathElement {
    #[js::import = "$0.startOffset"]
pub fn start_offset(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `method` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/method)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextPathElement`*"]
impl SvgTextPathElement {
    #[js::import = "$0.method"]
pub fn method(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `spacing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/spacing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgTextPathElement`*"]
impl SvgTextPathElement {
    #[js::import = "$0.spacing"]
pub fn spacing(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTextPathElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgTextPathElement`*"]
impl SvgTextPathElement {
    #[js::import = "$0.href"]
pub fn href(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
impl SvgTextPathElement {
    #[doc = "The `SVGTextPathElement.TEXTPATH_METHODTYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextPathElement`*"]
    pub const TEXTPATH_METHODTYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGTextPathElement.TEXTPATH_METHODTYPE_ALIGN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextPathElement`*"]
    pub const TEXTPATH_METHODTYPE_ALIGN: u16 = 1u64 as u16;
    #[doc = "The `SVGTextPathElement.TEXTPATH_METHODTYPE_STRETCH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextPathElement`*"]
    pub const TEXTPATH_METHODTYPE_STRETCH: u16 = 2u64 as u16;
    #[doc = "The `SVGTextPathElement.TEXTPATH_SPACINGTYPE_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextPathElement`*"]
    pub const TEXTPATH_SPACINGTYPE_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGTextPathElement.TEXTPATH_SPACINGTYPE_AUTO` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextPathElement`*"]
    pub const TEXTPATH_SPACINGTYPE_AUTO: u16 = 1u64 as u16;
    #[doc = "The `SVGTextPathElement.TEXTPATH_SPACINGTYPE_EXACT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTextPathElement`*"]
    pub const TEXTPATH_SPACINGTYPE_EXACT: u16 = 2u64 as u16;
}
