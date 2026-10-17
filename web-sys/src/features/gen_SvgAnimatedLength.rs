#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedLength` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedLength(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgLength")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLength`*"]
impl SvgAnimatedLength {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> SvgLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLength")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLength/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgLength`*"]
impl SvgAnimatedLength {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> SvgLength { panic!("web-sys intrinsic") }
}
