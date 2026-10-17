#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssStyleDeclaration` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssStyleDeclaration(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `cssText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.cssText"]
pub fn css_text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `cssText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.cssText = $1"]
pub fn set_css_text(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssRule")]
    #[doc = "Getter for the `parentRule` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/parentRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssRule`, `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.parentRule"]
pub fn parent_rule(&self) -> Option<CssRule> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getPropertyPriority()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyPriority)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.getPropertyPriority($1)"]
pub fn get_property_priority(&self, property: &str) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `getPropertyValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.getPropertyValue($1)"]
pub fn get_property_value(&self, property: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeProperty()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/removeProperty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.removeProperty($1)"]
pub fn remove_property(&self, property: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setProperty()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.setProperty($1, $2)"]
pub fn set_property(&self, property: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setProperty()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.setProperty($1, $2, $3)"]
pub fn set_property_with_priority(&self, property: &str, value: &str, priority: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`*"]
impl CssStyleDeclaration {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
