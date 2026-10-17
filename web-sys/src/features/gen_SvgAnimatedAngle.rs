#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedAngle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedAngle`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedAngle(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAngle")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`, `SvgAnimatedAngle`*"]
impl SvgAnimatedAngle {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> SvgAngle { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAngle")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedAngle/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`, `SvgAnimatedAngle`*"]
impl SvgAnimatedAngle {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> SvgAngle { panic!("web-sys intrinsic") }
}
