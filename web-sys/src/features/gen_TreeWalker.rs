#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeWalker` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeWalker`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TreeWalker(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `root` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/root)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.root"]
pub fn root(&self) -> Node { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `whatToShow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/whatToShow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.whatToShow"]
pub fn what_to_show(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeFilter")]
    #[doc = "Getter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.filter"]
pub fn filter(&self) -> Option<NodeFilter> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `currentNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.currentNode"]
pub fn current_node(&self) -> Node { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Setter for the `currentNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/currentNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.currentNode = $1"]
pub fn set_current_node(&self, value: &Node) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `firstChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/firstChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.firstChild()"]
pub fn first_child(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `lastChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/lastChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.lastChild()"]
pub fn last_child(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `nextNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.nextNode()"]
pub fn next_node(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `nextSibling()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/nextSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.nextSibling()"]
pub fn next_sibling(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `parentNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/parentNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.parentNode()"]
pub fn parent_node(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `previousNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.previousNode()"]
pub fn previous_node(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `previousSibling()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker/previousSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `TreeWalker`*"]
impl TreeWalker {
    #[js::import = "$0.previousSibling()"]
pub fn previous_sibling(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
