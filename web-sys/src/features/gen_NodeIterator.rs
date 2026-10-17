#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NodeIterator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeIterator`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NodeIterator(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `root` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/root)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
impl NodeIterator {
    #[js::import = "$0.root"]
pub fn root(&self) -> Node { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `referenceNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/referenceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
impl NodeIterator {
    #[js::import = "$0.referenceNode"]
pub fn reference_node(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pointerBeforeReferenceNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/pointerBeforeReferenceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeIterator`*"]
impl NodeIterator {
    #[js::import = "$0.pointerBeforeReferenceNode"]
pub fn pointer_before_reference_node(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `whatToShow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/whatToShow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeIterator`*"]
impl NodeIterator {
    #[js::import = "$0.whatToShow"]
pub fn what_to_show(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeFilter")]
    #[doc = "Getter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeFilter`, `NodeIterator`*"]
impl NodeIterator {
    #[js::import = "$0.filter"]
pub fn filter(&self) -> Option<NodeFilter> { panic!("web-sys intrinsic") }
}
    #[doc = "The `detach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/detach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NodeIterator`*"]
impl NodeIterator {
    #[js::import = "$0.detach()"]
pub fn detach(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `nextNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/nextNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
impl NodeIterator {
    #[js::import = "$0.nextNode()"]
pub fn next_node(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `previousNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/previousNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeIterator`*"]
impl NodeIterator {
    #[js::import = "$0.previousNode()"]
pub fn previous_node(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
