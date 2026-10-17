#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathSegLinetoVerticalRel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalRel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalRel`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgPathSegLinetoVerticalRel(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalRel`*"]
impl SvgPathSegLinetoVerticalRel {
    #[js::import = "$0.y"]
pub fn y(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegLinetoVerticalRel/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathSegLinetoVerticalRel`*"]
impl SvgPathSegLinetoVerticalRel {
    #[js::import = "$0.y = $1"]
pub fn set_y(&self, value: f32) { panic!("web-sys intrinsic") }
}
