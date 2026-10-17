#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedTransformList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedTransformList(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgTransformList")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgTransformList`*"]
impl SvgAnimatedTransformList {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> SvgTransformList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransformList")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedTransformList/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgTransformList`*"]
impl SvgAnimatedTransformList {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> SvgTransformList { panic!("web-sys intrinsic") }
}
