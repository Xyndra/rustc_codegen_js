#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssImportRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssImportRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssImportRule(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssImportRule`*"]
impl CssImportRule {
    #[js::import = "$0.href"]
pub fn href(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaList")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssImportRule`, `MediaList`*"]
impl CssImportRule {
    #[js::import = "$0.media"]
pub fn media(&self) -> Option<MediaList> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CssStyleSheet")]
    #[doc = "Getter for the `styleSheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSImportRule/styleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssImportRule`, `CssStyleSheet`*"]
impl CssImportRule {
    #[js::import = "$0.styleSheet"]
pub fn style_sheet(&self) -> Option<CssStyleSheet> { panic!("web-sys intrinsic") }
}
