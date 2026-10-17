#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedRect` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedRect(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgRect")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgRect`*"]
impl SvgAnimatedRect {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> Option<SvgRect> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgRect")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedRect/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgRect`*"]
impl SvgAnimatedRect {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> Option<SvgRect> { panic!("web-sys intrinsic") }
}
