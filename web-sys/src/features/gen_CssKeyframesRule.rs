#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssKeyframesRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssKeyframesRule(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
impl CssKeyframesRule {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
impl CssKeyframesRule {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssRuleList")]
    #[doc = "Getter for the `cssRules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/cssRules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`, `CssRuleList`*"]
impl CssKeyframesRule {
    #[js::import = "$0.cssRules"]
pub fn css_rules(&self) -> CssRuleList { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/appendRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
impl CssKeyframesRule {
    #[js::import = "$0.appendRule($1)"]
pub fn append_rule(&self, rule: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/deleteRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframesRule`*"]
impl CssKeyframesRule {
    #[js::import = "$0.deleteRule($1)"]
pub fn delete_rule(&self, select: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssKeyframeRule")]
    #[doc = "The `findRule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/findRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssKeyframeRule`, `CssKeyframesRule`*"]
impl CssKeyframesRule {
    #[js::import = "$0.findRule($1)"]
pub fn find_rule(&self, select: &str) -> Option<CssKeyframeRule> { panic!("web-sys intrinsic") }
}
