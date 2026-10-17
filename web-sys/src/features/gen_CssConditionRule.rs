#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssConditionRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssConditionRule(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `conditionText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
impl CssConditionRule {
    #[js::import = "$0.conditionText"]
pub fn condition_text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `conditionText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssConditionRule`*"]
impl CssConditionRule {
    #[js::import = "$0.conditionText = $1"]
pub fn set_condition_text(&self, value: &str) { panic!("web-sys intrinsic") }
}
