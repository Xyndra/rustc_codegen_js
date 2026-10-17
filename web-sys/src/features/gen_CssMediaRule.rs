#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssMediaRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssMediaRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssMediaRule(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaList")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSMediaRule/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssMediaRule`, `MediaList`*"]
impl CssMediaRule {
    #[js::import = "$0.media"]
pub fn media(&self) -> MediaList { panic!("web-sys intrinsic") }
}
