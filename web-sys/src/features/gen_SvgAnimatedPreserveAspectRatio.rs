#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedPreserveAspectRatio` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedPreserveAspectRatio(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgPreserveAspectRatio")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPreserveAspectRatio`*"]
impl SvgAnimatedPreserveAspectRatio {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> SvgPreserveAspectRatio { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPreserveAspectRatio")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedPreserveAspectRatio/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgPreserveAspectRatio`*"]
impl SvgAnimatedPreserveAspectRatio {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> SvgPreserveAspectRatio { panic!("web-sys intrinsic") }
}
