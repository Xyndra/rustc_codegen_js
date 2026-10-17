#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssFontFaceRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFaceRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssFontFaceRule(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFaceRule/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssFontFaceRule`, `CssStyleDeclaration`*"]
impl CssFontFaceRule {
    #[js::import = "$0.style"]
pub fn style(&self) -> CssStyleDeclaration { panic!("web-sys intrinsic") }
}
