#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NamedNodeMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NamedNodeMap`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NamedNodeMap(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `getNamedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.getNamedItem($1)"]
pub fn get_named_item(&self, name: &str) -> Option<Attr> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `getNamedItemNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItemNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.getNamedItemNS($1, $2)"]
pub fn get_named_item_ns(&self, namespace_uri: Option<&str>, local_name: &str) -> Option<Attr> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<Attr> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `removeNamedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.removeNamedItem($1)"]
pub fn remove_named_item(&self, name: &str) -> Result<Attr, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `removeNamedItemNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItemNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.removeNamedItemNS($1, $2)"]
pub fn remove_named_item_ns(&self, namespace_uri: Option<&str>, local_name: &str) -> Result<Attr, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `setNamedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.setNamedItem($1)"]
pub fn set_named_item(&self, arg: &Attr) -> Result<Option<Attr>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "The `setNamedItemNS()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItemNS)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.setNamedItemNS($1)"]
pub fn set_named_item_ns(&self, arg: &Attr) -> Result<Option<Attr>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.None($1)"]
pub fn get_with_name(&self, name: &str) -> Option<Attr> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Attr")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Attr`, `NamedNodeMap`*"]
impl NamedNodeMap {
    #[js::import = "$0.None($1)"]
pub fn get_with_index(&self, index: u32) -> Option<Attr> { panic!("web-sys intrinsic") }
}
