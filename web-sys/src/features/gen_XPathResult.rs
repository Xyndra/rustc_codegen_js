#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XPathResult` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
#[js::opaque]
#[derive(Copy)]
pub struct XPathResult(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `resultType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/resultType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
impl XPathResult {
    #[js::import = "$0.resultType"]
pub fn result_type(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `numberValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/numberValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
impl XPathResult {
    #[js::import = "$0.numberValue"]
pub fn number_value(&self) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `stringValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/stringValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
impl XPathResult {
    #[js::import = "$0.stringValue"]
pub fn string_value(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `booleanValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/booleanValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
impl XPathResult {
    #[js::import = "$0.booleanValue"]
pub fn boolean_value(&self) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `singleNodeValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/singleNodeValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XPathResult`*"]
impl XPathResult {
    #[js::import = "$0.singleNodeValue"]
pub fn single_node_value(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `invalidIteratorState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/invalidIteratorState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
impl XPathResult {
    #[js::import = "$0.invalidIteratorState"]
pub fn invalid_iterator_state(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `snapshotLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
impl XPathResult {
    #[js::import = "$0.snapshotLength"]
pub fn snapshot_length(&self) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `iterateNext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/iterateNext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XPathResult`*"]
impl XPathResult {
    #[js::import = "$0.iterateNext()"]
pub fn iterate_next(&self) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `snapshotItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XPathResult/snapshotItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XPathResult`*"]
impl XPathResult {
    #[js::import = "$0.snapshotItem($1)"]
pub fn snapshot_item(&self, index: u32) -> Result<Option<Node>, JsValue> { panic!("web-sys intrinsic") }
}
impl XPathResult {
    #[doc = "The `XPathResult.ANY_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const ANY_TYPE: u16 = 0i64 as u16;
    #[doc = "The `XPathResult.NUMBER_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const NUMBER_TYPE: u16 = 1u64 as u16;
    #[doc = "The `XPathResult.STRING_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const STRING_TYPE: u16 = 2u64 as u16;
    #[doc = "The `XPathResult.BOOLEAN_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const BOOLEAN_TYPE: u16 = 3u64 as u16;
    #[doc = "The `XPathResult.UNORDERED_NODE_ITERATOR_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const UNORDERED_NODE_ITERATOR_TYPE: u16 = 4u64 as u16;
    #[doc = "The `XPathResult.ORDERED_NODE_ITERATOR_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const ORDERED_NODE_ITERATOR_TYPE: u16 = 5u64 as u16;
    #[doc = "The `XPathResult.UNORDERED_NODE_SNAPSHOT_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const UNORDERED_NODE_SNAPSHOT_TYPE: u16 = 6u64 as u16;
    #[doc = "The `XPathResult.ORDERED_NODE_SNAPSHOT_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const ORDERED_NODE_SNAPSHOT_TYPE: u16 = 7u64 as u16;
    #[doc = "The `XPathResult.ANY_UNORDERED_NODE_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const ANY_UNORDERED_NODE_TYPE: u16 = 8u64 as u16;
    #[doc = "The `XPathResult.FIRST_ORDERED_NODE_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathResult`*"]
    pub const FIRST_ORDERED_NODE_TYPE: u16 = 9u64 as u16;
}
