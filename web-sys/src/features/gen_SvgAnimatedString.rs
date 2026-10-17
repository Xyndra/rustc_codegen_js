#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimatedString` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimatedString(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
impl SvgAnimatedString {
    #[js::import = "$0.baseVal"]
pub fn base_val(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `baseVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/baseVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
impl SvgAnimatedString {
    #[js::import = "$0.baseVal = $1"]
pub fn set_base_val(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `animVal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimatedString/animVal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`*"]
impl SvgAnimatedString {
    #[js::import = "$0.animVal"]
pub fn anim_val(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
