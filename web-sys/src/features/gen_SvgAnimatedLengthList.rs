#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedLengthList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLengthList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedLengthList(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgLengthList")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgLengthList`*"]
impl SvgAnimatedLengthList {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> SvgLengthList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLengthList")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedLengthList/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLengthList`, `SvgLengthList`*"]
impl SvgAnimatedLengthList {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> SvgLengthList { panic!("web-sys intrinsic") }
}
