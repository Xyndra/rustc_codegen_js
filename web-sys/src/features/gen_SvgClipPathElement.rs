#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgClipPathElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgClipPathElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgClipPathElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedEnumeration")]
    #[doc = "Getter for the `clipPathUnits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/clipPathUnits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`, `SvgClipPathElement`*"]
impl SvgClipPathElement {
    #[js::import = "$0.clipPathUnits"]
pub fn clip_path_units(&self) -> SvgAnimatedEnumeration { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGClipPathElement/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgClipPathElement`*"]
impl SvgClipPathElement {
    #[js::import = "$0.transform"]
pub fn transform(&self) -> SvgAnimatedTransformList { panic!("web-sys intrinsic") }
}
