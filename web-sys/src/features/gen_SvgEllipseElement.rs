#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgEllipseElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgEllipseElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgEllipseElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cx` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/cx)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*"]
impl SvgEllipseElement {
    #[js::import = "$0.cx"]
pub fn cx(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/cy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*"]
impl SvgEllipseElement {
    #[js::import = "$0.cy"]
pub fn cy(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `rx` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/rx)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*"]
impl SvgEllipseElement {
    #[js::import = "$0.rx"]
pub fn rx(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `ry` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGEllipseElement/ry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgEllipseElement`*"]
impl SvgEllipseElement {
    #[js::import = "$0.ry"]
pub fn ry(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
