#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheetList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StyleSheetList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheetList`*"]
impl StyleSheetList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StyleSheet")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheetList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*"]
impl StyleSheetList {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<StyleSheet> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StyleSheet")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`, `StyleSheetList`*"]
impl StyleSheetList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<StyleSheet> { panic!("web-sys intrinsic") }
}
