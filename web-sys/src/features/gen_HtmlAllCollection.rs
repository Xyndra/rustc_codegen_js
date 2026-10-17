#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlAllCollection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlAllCollection(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
impl HtmlAllCollection {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*"]
impl HtmlAllCollection {
    #[js::import = "$0.item($1)"]
pub fn item_with_index(&self, index: u32) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
impl HtmlAllCollection {
    #[js::import = "$0.item($1)"]
pub fn item_with_name(&self, name: &str) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAllCollection/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
impl HtmlAllCollection {
    #[js::import = "$0.namedItem($1)"]
pub fn named_item(&self, name: &str) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`, `Node`*"]
impl HtmlAllCollection {
    #[js::import = "$0.None($1)"]
pub fn get_with_index(&self, index: u32) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`*"]
impl HtmlAllCollection {
    #[js::import = "$0.None($1)"]
pub fn get_with_name(&self, name: &str) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
