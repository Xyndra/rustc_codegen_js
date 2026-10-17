#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssGroupingRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssGroupingRule(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CssRuleList")]
    #[doc = "Getter for the `cssRules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/cssRules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`, `CssRuleList`*"]
impl CssGroupingRule {
    #[js::import = "$0.cssRules"]
pub fn css_rules(&self) -> CssRuleList { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/deleteRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`*"]
impl CssGroupingRule {
    #[js::import = "$0.deleteRule($1)"]
pub fn delete_rule(&self, index: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`*"]
impl CssGroupingRule {
    #[js::import = "$0.insertRule($1)"]
pub fn insert_rule(&self, rule: &str) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssGroupingRule`*"]
impl CssGroupingRule {
    #[js::import = "$0.insertRule($1, $2)"]
pub fn insert_rule_with_index(&self, rule: &str, index: u32) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
