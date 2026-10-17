#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Node` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Node(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `nodeType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.nodeType"]
pub fn node_type(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `nodeName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.nodeName"]
pub fn node_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `baseURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/baseURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.baseURI"]
pub fn base_uri(&self) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isConnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isConnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.isConnected"]
pub fn is_connected(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `ownerDocument` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/ownerDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Node`*"]
impl Node {
    #[js::import = "$0.ownerDocument"]
pub fn owner_document(&self) -> Option<Document> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `parentNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.parentNode"]
pub fn parent_node(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `parentElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/parentElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `Node`*"]
impl Node {
    #[js::import = "$0.parentElement"]
pub fn parent_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `childNodes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/childNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `NodeList`*"]
impl Node {
    #[js::import = "$0.childNodes"]
pub fn child_nodes(&self) -> NodeList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `firstChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/firstChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.firstChild"]
pub fn first_child(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lastChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lastChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.lastChild"]
pub fn last_child(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `previousSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/previousSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.previousSibling"]
pub fn previous_sibling(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `nextSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nextSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.nextSibling"]
pub fn next_sibling(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `nodeValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.nodeValue"]
pub fn node_value(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `nodeValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/nodeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.nodeValue = $1"]
pub fn set_node_value(&self, value: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `textContent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.textContent"]
pub fn text_content(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `textContent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.textContent = $1"]
pub fn set_text_content(&self, value: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/appendChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.appendChild($1)"]
pub fn append_child(&self, node: &Node) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `cloneNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.cloneNode()"]
pub fn clone_node(&self) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `cloneNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/cloneNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.cloneNode($1)"]
pub fn clone_node_with_deep(&self, deep: bool) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `compareDocumentPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/compareDocumentPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.compareDocumentPosition($1)"]
pub fn compare_document_position(&self, other: &Node) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "The `contains()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/contains)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.contains($1)"]
pub fn contains(&self, other: Option<&Node>) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRootNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.getRootNode()"]
pub fn get_root_node(&self) -> Node { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "GetRootNodeOptions")]
    #[doc = "The `getRootNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/getRootNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GetRootNodeOptions`, `Node`*"]
impl Node {
    #[js::import = "$0.getRootNode($1)"]
pub fn get_root_node_with_options(&self, options: &GetRootNodeOptions) -> Node { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasChildNodes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/hasChildNodes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.hasChildNodes()"]
pub fn has_child_nodes(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/insertBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.insertBefore($1, $2)"]
pub fn insert_before(&self, node: &Node, child: Option<&Node>) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `isDefaultNamespace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isDefaultNamespace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.isDefaultNamespace($1)"]
pub fn is_default_namespace(&self, namespace: Option<&str>) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `isEqualNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isEqualNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.isEqualNode($1)"]
pub fn is_equal_node(&self, node: Option<&Node>) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `isSameNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.isSameNode($1)"]
pub fn is_same_node(&self, node: Option<&Node>) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `lookupNamespaceURI()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupNamespaceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.lookupNamespaceURI($1)"]
pub fn lookup_namespace_uri(&self, prefix: Option<&str>) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `lookupPrefix()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/lookupPrefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.lookupPrefix($1)"]
pub fn lookup_prefix(&self, namespace: Option<&str>) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `normalize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/normalize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.normalize()"]
pub fn normalize(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/removeChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.removeChild($1)"]
pub fn remove_child(&self, child: &Node) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChild()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/replaceChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
impl Node {
    #[js::import = "$0.replaceChild($1, $2)"]
pub fn replace_child(&self, node: &Node, child: &Node) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
impl Node {
    #[doc = "The `Node.ELEMENT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const ELEMENT_NODE: u16 = 1u64 as u16;
    #[doc = "The `Node.ATTRIBUTE_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const ATTRIBUTE_NODE: u16 = 2u64 as u16;
    #[doc = "The `Node.TEXT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const TEXT_NODE: u16 = 3u64 as u16;
    #[doc = "The `Node.CDATA_SECTION_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const CDATA_SECTION_NODE: u16 = 4u64 as u16;
    #[doc = "The `Node.ENTITY_REFERENCE_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const ENTITY_REFERENCE_NODE: u16 = 5u64 as u16;
    #[doc = "The `Node.ENTITY_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const ENTITY_NODE: u16 = 6u64 as u16;
    #[doc = "The `Node.PROCESSING_INSTRUCTION_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const PROCESSING_INSTRUCTION_NODE: u16 = 7u64 as u16;
    #[doc = "The `Node.COMMENT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const COMMENT_NODE: u16 = 8u64 as u16;
    #[doc = "The `Node.DOCUMENT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_NODE: u16 = 9u64 as u16;
    #[doc = "The `Node.DOCUMENT_TYPE_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_TYPE_NODE: u16 = 10u64 as u16;
    #[doc = "The `Node.DOCUMENT_FRAGMENT_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_FRAGMENT_NODE: u16 = 11u64 as u16;
    #[doc = "The `Node.NOTATION_NODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const NOTATION_NODE: u16 = 12u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_DISCONNECTED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_DISCONNECTED: u16 = 1u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_PRECEDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_PRECEDING: u16 = 2u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_FOLLOWING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_FOLLOWING: u16 = 4u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_CONTAINS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_CONTAINS: u16 = 8u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_CONTAINED_BY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_CONTAINED_BY: u16 = 16u64 as u16;
    #[doc = "The `Node.DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`*"]
    pub const DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC: u16 = 32u64 as u16;
}
