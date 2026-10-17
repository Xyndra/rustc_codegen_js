#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgCircleElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgCircleElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgCircleElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cx` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cx)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*"]
impl SvgCircleElement {
    #[js::import = "$0.cx"]
pub fn cx(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `cy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/cy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*"]
impl SvgCircleElement {
    #[js::import = "$0.cy"]
pub fn cy(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `r` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGCircleElement/r)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgCircleElement`*"]
impl SvgCircleElement {
    #[js::import = "$0.r"]
pub fn r(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
