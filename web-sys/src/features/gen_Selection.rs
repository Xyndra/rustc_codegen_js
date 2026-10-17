#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Selection` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Selection(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `anchorNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.anchorNode"]
pub fn anchor_node(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `anchorOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.anchorOffset"]
pub fn anchor_offset(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `focusNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.focusNode"]
pub fn focus_node(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `focusOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.focusOffset"]
pub fn focus_offset(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isCollapsed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/isCollapsed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.isCollapsed"]
pub fn is_collapsed(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rangeCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/rangeCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.rangeCount"]
pub fn range_count(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `caretBidiLevel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/caretBidiLevel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.caretBidiLevel"]
pub fn caret_bidi_level(&self) -> Result<Option<i16>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `caretBidiLevel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/caretBidiLevel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.caretBidiLevel = $1"]
pub fn set_caret_bidi_level(&self, value: Option<i16>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Range")]
    #[doc = "The `addRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/addRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`, `Selection`*"]
impl Selection {
    #[js::import = "$0.addRange($1)"]
pub fn add_range(&self, range: &Range) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `collapse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.collapse($1)"]
pub fn collapse(&self, node: Option<&Node>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `collapse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.collapse($1, $2)"]
pub fn collapse_with_offset(&self, node: Option<&Node>, offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `collapseToEnd()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.collapseToEnd()"]
pub fn collapse_to_end(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `collapseToStart()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.collapseToStart()"]
pub fn collapse_to_start(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `containsNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.containsNode($1)"]
pub fn contains_node(&self, node: &Node) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `containsNode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.containsNode($1, $2)"]
pub fn contains_node_with_allow_partial_containment(&self, node: &Node, allow_partial_containment: bool) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteFromDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/deleteFromDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.deleteFromDocument()"]
pub fn delete_from_document(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `empty()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/empty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.empty()"]
pub fn empty(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `extend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.extend($1)"]
pub fn extend(&self, node: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `extend()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.extend($1, $2)"]
pub fn extend_with_offset(&self, node: &Node, offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Range")]
    #[doc = "The `getRangeAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/getRangeAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`, `Selection`*"]
impl Selection {
    #[js::import = "$0.getRangeAt($1)"]
pub fn get_range_at(&self, index: u32) -> Result<Range, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `modify()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/modify)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.modify($1, $2, $3)"]
pub fn modify(&self, alter: &str, direction: &str, granularity: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeAllRanges()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeAllRanges)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`*"]
impl Selection {
    #[js::import = "$0.removeAllRanges()"]
pub fn remove_all_ranges(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Range")]
    #[doc = "The `removeRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Range`, `Selection`*"]
impl Selection {
    #[js::import = "$0.removeRange($1)"]
pub fn remove_range(&self, range: &Range) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `selectAllChildren()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/selectAllChildren)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.selectAllChildren($1)"]
pub fn select_all_children(&self, node: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `setBaseAndExtent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setBaseAndExtent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.setBaseAndExtent($1, $2, $3, $4)"]
pub fn set_base_and_extent(&self, anchor_node: &Node, anchor_offset: u32, focus_node: &Node, focus_offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `setPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.setPosition($1)"]
pub fn set_position(&self, node: Option<&Node>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `setPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `Selection`*"]
impl Selection {
    #[js::import = "$0.setPosition($1, $2)"]
pub fn set_position_with_offset(&self, node: Option<&Node>, offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
