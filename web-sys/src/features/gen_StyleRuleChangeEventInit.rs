#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleRuleChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StyleRuleChangeEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &StyleRuleChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &StyleRuleChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &StyleRuleChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &StyleRuleChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &StyleRuleChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &StyleRuleChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssRule")]
    #[doc = "Get the `rule` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `StyleRuleChangeEventInit`*"]
#[js::import = "None"]
pub fn get_rule(this: &StyleRuleChangeEventInit) -> Option<CssRule> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssRule")]
    #[doc = "Change the `rule` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `StyleRuleChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_rule(this: &StyleRuleChangeEventInit, val: Option<&CssRule>) { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Get the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleRuleChangeEventInit`*"]
#[js::import = "None"]
pub fn get_stylesheet(this: &StyleRuleChangeEventInit) -> Option<CssStyleSheet> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Change the `stylesheet` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleSheet`, `StyleRuleChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_stylesheet(this: &StyleRuleChangeEventInit, val: Option<&CssStyleSheet>) { panic!("web-sys intrinsic") }
impl StyleRuleChangeEventInit {
    #[doc = "Construct a new `StyleRuleChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleRuleChangeEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "CssRule")]
    #[deprecated = "Use `set_rule()` instead."]
#[js::import = "None"]
pub fn rule(&mut self, val: Option<&CssRule>) -> &mut Self { self.set_rule(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "CssStyleSheet")]
    #[deprecated = "Use `set_stylesheet()` instead."]
#[js::import = "None"]
pub fn stylesheet(&mut self, val: Option<&CssStyleSheet>) -> &mut Self { self.set_stylesheet(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for StyleRuleChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
