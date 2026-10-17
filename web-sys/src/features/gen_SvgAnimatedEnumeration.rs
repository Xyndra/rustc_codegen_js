#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedEnumeration` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedEnumeration(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
impl SvgAnimatedEnumeration {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
impl SvgAnimatedEnumeration {
    #[js::import = "$0.baseVal = $1"]
pub fn set_base_val(&self, value: u16) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedEnumeration/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedEnumeration`*"]
impl SvgAnimatedEnumeration {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> u16 { panic!("web-sys intrinsic") }
}
