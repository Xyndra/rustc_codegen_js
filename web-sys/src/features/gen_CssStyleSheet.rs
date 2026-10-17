#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssStyleSheet` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssStyleSheet(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CssRule")]
    #[doc = "Getter for the `ownerRule` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/ownerRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `CssStyleSheet`*"]
impl CssStyleSheet {
    #[js::import = "$0.ownerRule"]
pub fn owner_rule(&self) -> Option<CssRule> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssRuleList")]
    #[doc = "Getter for the `cssRules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/cssRules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRuleList`, `CssStyleSheet`*"]
impl CssStyleSheet {
    #[js::import = "$0.cssRules"]
pub fn css_rules(&self) -> Result<CssRuleList, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new CssStyleSheet(..)` constructor, creating a new instance of `CssStyleSheet`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/CSSStyleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
impl CssStyleSheet {
    #[js::import = "new CSSStyleSheet()"]
pub fn new() -> Result<CssStyleSheet, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/deleteRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
impl CssStyleSheet {
    #[js::import = "$0.deleteRule($1)"]
pub fn delete_rule(&self, index: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
impl CssStyleSheet {
    #[js::import = "$0.insertRule($1)"]
pub fn insert_rule(&self, rule: &str) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
impl CssStyleSheet {
    #[js::import = "$0.insertRule($1, $2)"]
pub fn insert_rule_with_index(&self, rule: &str, index: u32) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/replace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
impl CssStyleSheet {
    #[js::import = "$0.replace($1)"]
pub fn replace(&self, text: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceSync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/replaceSync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`*"]
impl CssStyleSheet {
    #[js::import = "$0.replaceSync($1)"]
pub fn replace_sync(&self, text: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
