#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssStyleRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssStyleRule(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `selectorText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleRule`*"]
impl CssStyleRule {
    #[js::import = "$0.selectorText"]
pub fn selector_text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `selectorText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleRule`*"]
impl CssStyleRule {
    #[js::import = "$0.selectorText = $1"]
pub fn set_selector_text(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`, `CssStyleRule`*"]
impl CssStyleRule {
    #[js::import = "$0.style"]
pub fn style(&self) -> CssStyleDeclaration { panic!("web-sys intrinsic") }
}
