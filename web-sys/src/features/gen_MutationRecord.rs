#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationRecord` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MutationRecord(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
impl MutationRecord {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
impl MutationRecord {
    #[js::import = "$0.target"]
pub fn target(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `addedNodes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/addedNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `NodeList`*"]
impl MutationRecord {
    #[js::import = "$0.addedNodes"]
pub fn added_nodes(&self) -> NodeList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `removedNodes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/removedNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `NodeList`*"]
impl MutationRecord {
    #[js::import = "$0.removedNodes"]
pub fn removed_nodes(&self) -> NodeList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `previousSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/previousSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
impl MutationRecord {
    #[js::import = "$0.previousSibling"]
pub fn previous_sibling(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `nextSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/nextSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`, `Node`*"]
impl MutationRecord {
    #[js::import = "$0.nextSibling"]
pub fn next_sibling(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `attributeName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
impl MutationRecord {
    #[js::import = "$0.attributeName"]
pub fn attribute_name(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `attributeNamespace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeNamespace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
impl MutationRecord {
    #[js::import = "$0.attributeNamespace"]
pub fn attribute_namespace(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/oldValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationRecord`*"]
impl MutationRecord {
    #[js::import = "$0.oldValue"]
pub fn old_value(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
