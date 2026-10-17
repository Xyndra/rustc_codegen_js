#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DocumentFragment` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DocumentFragment(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `children` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/children)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `HtmlCollection`*"]
impl DocumentFragment {
    #[js::import = "$0.children"]
pub fn children(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `firstElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/firstElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
impl DocumentFragment {
    #[js::import = "$0.firstElementChild"]
pub fn first_element_child(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `lastElementChild` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/lastElementChild)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
impl DocumentFragment {
    #[js::import = "$0.lastElementChild"]
pub fn last_element_child(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `childElementCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/childElementCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.childElementCount"]
pub fn child_element_count(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DocumentFragment(..)` constructor, creating a new instance of `DocumentFragment`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/DocumentFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "new DocumentFragment()"]
pub fn new() -> Result<DocumentFragment, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `getElementById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/getElementById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
impl DocumentFragment {
    #[js::import = "$0.getElementById($1)"]
pub fn get_element_by_id(&self, element_id: &str) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `querySelector()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelector)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Element`*"]
impl DocumentFragment {
    #[js::import = "$0.querySelector($1)"]
pub fn query_selector(&self, selectors: &str) -> Result<Option<Element>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "The `querySelectorAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/querySelectorAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `NodeList`*"]
impl DocumentFragment {
    #[js::import = "$0.querySelectorAll($1)"]
pub fn query_selector_all(&self, selectors: &str) -> Result<NodeList, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1)"]
pub fn append_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append()"]
pub fn append_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1)"]
pub fn append_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2)"]
pub fn append_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3)"]
pub fn append_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3, $4)"]
pub fn append_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3, $4, $5)"]
pub fn append_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6)"]
pub fn append_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6, $7)"]
pub fn append_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1)"]
pub fn append_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append()"]
pub fn append_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1)"]
pub fn append_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2)"]
pub fn append_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3)"]
pub fn append_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3, $4)"]
pub fn append_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3, $4, $5)"]
pub fn append_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6)"]
pub fn append_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.append($1, $2, $3, $4, $5, $6, $7)"]
pub fn append_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend()"]
pub fn prepend_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2)"]
pub fn prepend_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3)"]
pub fn prepend_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3, $4)"]
pub fn prepend_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5)"]
pub fn prepend_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6)"]
pub fn prepend_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6, $7)"]
pub fn prepend_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend()"]
pub fn prepend_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1)"]
pub fn prepend_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2)"]
pub fn prepend_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3)"]
pub fn prepend_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3, $4)"]
pub fn prepend_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5)"]
pub fn prepend_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6)"]
pub fn prepend_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prepend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/prepend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.prepend($1, $2, $3, $4, $5, $6, $7)"]
pub fn prepend_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_node(&self, nodes: &::js_sys::Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren()"]
pub fn replace_children_with_node_0(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_node_1(&self, nodes_1: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2)"]
pub fn replace_children_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3)"]
pub fn replace_children_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4)"]
pub fn replace_children_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5)"]
pub fn replace_children_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6)"]
pub fn replace_children_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_children_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_str(&self, nodes: &::js_sys::Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren()"]
pub fn replace_children_with_str_0(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1)"]
pub fn replace_children_with_str_1(&self, nodes_1: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2)"]
pub fn replace_children_with_str_2(&self, nodes_1: &str, nodes_2: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3)"]
pub fn replace_children_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4)"]
pub fn replace_children_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5)"]
pub fn replace_children_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6)"]
pub fn replace_children_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentFragment/replaceChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`*"]
impl DocumentFragment {
    #[js::import = "$0.replaceChildren($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_children_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) { panic!("web-sys intrinsic") }
}
