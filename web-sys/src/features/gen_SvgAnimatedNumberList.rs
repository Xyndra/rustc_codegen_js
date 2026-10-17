#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedNumberList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedNumberList(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgNumberList")]
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*"]
impl SvgAnimatedNumberList {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> SvgNumberList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgNumberList")]
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedNumberList/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumberList`, `SvgNumberList`*"]
impl SvgAnimatedNumberList {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> SvgNumberList { panic!("web-sys intrinsic") }
}
