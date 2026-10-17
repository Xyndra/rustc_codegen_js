#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedInteger` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedInteger(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
impl SvgAnimatedInteger {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
impl SvgAnimatedInteger {
    #[js::import = "$0.baseVal = $1"]
pub fn set_base_val(&self, value: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedInteger/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedInteger`*"]
impl SvgAnimatedInteger {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> i32 { panic!("web-sys intrinsic") }
}
