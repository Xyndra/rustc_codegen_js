#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlStyleElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlStyleElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
impl HtmlStyleElement {
    #[js::import = "$0.disabled"]
pub fn disabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
impl HtmlStyleElement {
    #[js::import = "$0.disabled = $1"]
pub fn set_disabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
impl HtmlStyleElement {
    #[js::import = "$0.media"]
pub fn media(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
impl HtmlStyleElement {
    #[js::import = "$0.media = $1"]
pub fn set_media(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
impl HtmlStyleElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`*"]
impl HtmlStyleElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StyleSheet")]
    #[doc = "Getter for the `sheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLStyleElement/sheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlStyleElement`, `StyleSheet`*"]
impl HtmlStyleElement {
    #[js::import = "$0.sheet"]
pub fn sheet(&self) -> Option<StyleSheet> { panic!("web-sys intrinsic") }
}
