#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssKeyframeRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssKeyframeRule(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `keyText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
impl CssKeyframeRule {
    #[js::import = "$0.keyText"]
pub fn key_text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `keyText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`*"]
impl CssKeyframeRule {
    #[js::import = "$0.keyText = $1"]
pub fn set_key_text(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssStyleDeclaration")]
    #[doc = "Getter for the `style` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/style)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`, `CssStyleDeclaration`*"]
impl CssKeyframeRule {
    #[js::import = "$0.style"]
pub fn style(&self) -> CssStyleDeclaration { panic!("web-sys intrinsic") }
}
