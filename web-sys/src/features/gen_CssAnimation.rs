#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssAnimation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssAnimation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssAnimation(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `animationName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSAnimation/animationName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssAnimation`*"]
impl CssAnimation {
    #[js::import = "$0.animationName"]
pub fn animation_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
