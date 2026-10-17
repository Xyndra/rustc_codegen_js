#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CharacterData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CharacterData(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.data = $1"]
pub fn set_data(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `previousElementSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/previousElementSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`, `Element`*"]
impl CharacterData {
    #[js::import = "$0.previousElementSibling"]
pub fn previous_element_sibling(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `nextElementSibling` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/nextElementSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`, `Element`*"]
impl CharacterData {
    #[js::import = "$0.nextElementSibling"]
pub fn next_element_sibling(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/appendData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.appendData($1)"]
pub fn append_data(&self, data: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/deleteData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.deleteData($1, $2)"]
pub fn delete_data(&self, offset: u32, count: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/insertData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.insertData($1, $2)"]
pub fn insert_data(&self, offset: u32, data: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceData($1, $2, $3)"]
pub fn replace_data(&self, offset: u32, count: u32, data: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `substringData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/substringData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.substringData($1, $2)"]
pub fn substring_data(&self, offset: u32, count: u32) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1)"]
pub fn after_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after()"]
pub fn after_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1)"]
pub fn after_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2)"]
pub fn after_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3)"]
pub fn after_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3, $4)"]
pub fn after_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3, $4, $5)"]
pub fn after_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3, $4, $5, $6)"]
pub fn after_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3, $4, $5, $6, $7)"]
pub fn after_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1)"]
pub fn after_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after()"]
pub fn after_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1)"]
pub fn after_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2)"]
pub fn after_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3)"]
pub fn after_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3, $4)"]
pub fn after_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3, $4, $5)"]
pub fn after_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3, $4, $5, $6)"]
pub fn after_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `after()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.after($1, $2, $3, $4, $5, $6, $7)"]
pub fn after_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1)"]
pub fn before_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before()"]
pub fn before_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1)"]
pub fn before_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2)"]
pub fn before_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3)"]
pub fn before_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3, $4)"]
pub fn before_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3, $4, $5)"]
pub fn before_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3, $4, $5, $6)"]
pub fn before_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3, $4, $5, $6, $7)"]
pub fn before_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1)"]
pub fn before_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before()"]
pub fn before_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1)"]
pub fn before_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2)"]
pub fn before_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3)"]
pub fn before_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3, $4)"]
pub fn before_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3, $4, $5)"]
pub fn before_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3, $4, $5, $6)"]
pub fn before_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `before()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.before($1, $2, $3, $4, $5, $6, $7)"]
pub fn before_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.remove()"]
pub fn remove(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1)"]
pub fn replace_with_with_node(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith()"]
pub fn replace_with_with_node_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1)"]
pub fn replace_with_with_node_1(&self, nodes_1: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2)"]
pub fn replace_with_with_node_2(&self, nodes_1: &Node, nodes_2: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3)"]
pub fn replace_with_with_node_3(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3, $4)"]
pub fn replace_with_with_node_4(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5)"]
pub fn replace_with_with_node_5(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5, $6)"]
pub fn replace_with_with_node_6(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_with_with_node_7(&self, nodes_1: &Node, nodes_2: &Node, nodes_3: &Node, nodes_4: &Node, nodes_5: &Node, nodes_6: &Node, nodes_7: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1)"]
pub fn replace_with_with_str(&self, nodes: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith()"]
pub fn replace_with_with_str_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1)"]
pub fn replace_with_with_str_1(&self, nodes_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2)"]
pub fn replace_with_with_str_2(&self, nodes_1: &str, nodes_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3)"]
pub fn replace_with_with_str_3(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3, $4)"]
pub fn replace_with_with_str_4(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5)"]
pub fn replace_with_with_str_5(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5, $6)"]
pub fn replace_with_with_str_6(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `replaceWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CharacterData`*"]
impl CharacterData {
    #[js::import = "$0.replaceWith($1, $2, $3, $4, $5, $6, $7)"]
pub fn replace_with_with_str_7(&self, nodes_1: &str, nodes_2: &str, nodes_3: &str, nodes_4: &str, nodes_5: &str, nodes_6: &str, nodes_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
