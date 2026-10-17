#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedBoolean` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedBoolean(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*"]
impl SvgAnimatedBoolean {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*"]
impl SvgAnimatedBoolean {
    #[js::import = "$0.baseVal = $1"]
pub fn set_base_val(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedBoolean/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedBoolean`*"]
impl SvgAnimatedBoolean {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> bool { panic!("web-sys intrinsic") }
}
