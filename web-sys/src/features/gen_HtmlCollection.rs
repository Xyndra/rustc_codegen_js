#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlCollection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlCollection(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`*"]
impl HtmlCollection {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
impl HtmlCollection {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCollection/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
impl HtmlCollection {
    #[js::import = "$0.namedItem($1)"]
pub fn named_item(&self, name: &str) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
impl HtmlCollection {
    #[js::import = "$0.None($1)"]
pub fn get_with_index(&self, index: u32) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HtmlCollection`*"]
impl HtmlCollection {
    #[js::import = "$0.None($1)"]
pub fn get_with_name(&self, name: &str) -> Option<Element> { panic!("web-sys intrinsic") }
}
