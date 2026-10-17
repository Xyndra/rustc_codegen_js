#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Range` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Range(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `startContainer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/startContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.startContainer"]
pub fn start_container(&self) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `startOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/startOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "$0.startOffset"]
pub fn start_offset(&self) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `endContainer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/endContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.endContainer"]
pub fn end_container(&self) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `endOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/endOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "$0.endOffset"]
pub fn end_offset(&self) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `collapsed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapsed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "$0.collapsed"]
pub fn collapsed(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `commonAncestorContainer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/commonAncestorContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.commonAncestorContainer"]
pub fn common_ancestor_container(&self) -> Result<Node, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Range(..)` constructor, creating a new instance of `Range`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/Range)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "new Range()"]
pub fn new() -> Result<Range, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DocumentFragment")]
    #[doc = "The `cloneContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*"]
impl Range {
    #[js::import = "$0.cloneContents()"]
pub fn clone_contents(&self) -> Result<DocumentFragment, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `cloneRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/cloneRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "$0.cloneRange()"]
pub fn clone_range(&self) -> Range { panic!("web-sys intrinsic") }
}
    #[doc = "The `collapse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "$0.collapse()"]
pub fn collapse(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `collapse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "$0.collapse($1)"]
pub fn collapse_with_to_start(&self, to_start: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `compareBoundaryPoints()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/compareBoundaryPoints)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "$0.compareBoundaryPoints($1, $2)"]
pub fn compare_boundary_points(&self, how: u16, source_range: &Range) -> Result<i16, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `comparePoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/comparePoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.comparePoint($1, $2)"]
pub fn compare_point(&self, node: &Node, offset: u32) -> Result<i16, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DocumentFragment")]
    #[doc = "The `createContextualFragment()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/createContextualFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*"]
impl Range {
    #[js::import = "$0.createContextualFragment($1)"]
pub fn create_contextual_fragment(&self, fragment: &str) -> Result<DocumentFragment, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/deleteContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "$0.deleteContents()"]
pub fn delete_contents(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `detach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/detach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
impl Range {
    #[js::import = "$0.detach()"]
pub fn detach(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DocumentFragment")]
    #[doc = "The `extractContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/extractContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `Range`*"]
impl Range {
    #[js::import = "$0.extractContents()"]
pub fn extract_contents(&self) -> Result<DocumentFragment, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRect")]
    #[doc = "The `getBoundingClientRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/getBoundingClientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`, `Range`*"]
impl Range {
    #[js::import = "$0.getBoundingClientRect()"]
pub fn get_bounding_client_rect(&self) -> DomRect { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRectList")]
    #[doc = "The `getClientRects()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/getClientRects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectList`, `Range`*"]
impl Range {
    #[js::import = "$0.getClientRects()"]
pub fn get_client_rects(&self) -> Option<DomRectList> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `insertNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/insertNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.insertNode($1)"]
pub fn insert_node(&self, node: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `intersectsNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/intersectsNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.intersectsNode($1)"]
pub fn intersects_node(&self, node: &Node) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `isPointInRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/isPointInRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.isPointInRange($1, $2)"]
pub fn is_point_in_range(&self, node: &Node, offset: u32) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `selectNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.selectNode($1)"]
pub fn select_node(&self, ref_node: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `selectNodeContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/selectNodeContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.selectNodeContents($1)"]
pub fn select_node_contents(&self, ref_node: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `setEnd()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.setEnd($1, $2)"]
pub fn set_end(&self, ref_node: &Node, offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `setEndAfter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndAfter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.setEndAfter($1)"]
pub fn set_end_after(&self, ref_node: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `setEndBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setEndBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.setEndBefore($1)"]
pub fn set_end_before(&self, ref_node: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `setStart()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.setStart($1, $2)"]
pub fn set_start(&self, ref_node: &Node, offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `setStartAfter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartAfter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.setStartAfter($1)"]
pub fn set_start_after(&self, ref_node: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `setStartBefore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/setStartBefore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.setStartBefore($1)"]
pub fn set_start_before(&self, ref_node: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `surroundContents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Range/surroundContents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Range`*"]
impl Range {
    #[js::import = "$0.surroundContents($1)"]
pub fn surround_contents(&self, new_parent: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl Range {
    #[doc = "The `Range.START_TO_START` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub const START_TO_START: u16 = 0i64 as u16;
    #[doc = "The `Range.START_TO_END` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub const START_TO_END: u16 = 1u64 as u16;
    #[doc = "The `Range.END_TO_END` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub const END_TO_END: u16 = 2u64 as u16;
    #[doc = "The `Range.END_TO_START` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`*"]
    pub const END_TO_START: u16 = 3u64 as u16;
}
