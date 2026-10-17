#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NodeList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NodeList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
impl NodeList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
impl NodeList {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
impl NodeList {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
impl NodeList {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
impl NodeList {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeList`*"]
impl NodeList {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
impl NodeList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<Node> { panic!("web-sys intrinsic") }
}
