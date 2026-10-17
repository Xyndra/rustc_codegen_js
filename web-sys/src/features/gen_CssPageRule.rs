#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssPageRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPageRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssPageRule(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssPageRule`, `CssStyleDeclaration`*"]
impl CssPageRule {
    #[js::import = "$0.style"]
pub fn style(&self) -> CssStyleDeclaration { panic!("web-sys intrinsic") }
}
