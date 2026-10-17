#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeView` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TreeView(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `rowCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/rowCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.rowCount"]
pub fn row_count(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DataTransfer")]
    #[doc = "The `canDrop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/canDrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*"]
impl TreeView {
    #[js::import = "$0.canDrop($1, $2, $3)"]
pub fn can_drop(&self, row: i32, orientation: i32, data_transfer: Option<&DataTransfer>) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DataTransfer")]
    #[doc = "The `drop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/drop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `TreeView`*"]
impl TreeView {
    #[js::import = "$0.drop($1, $2, $3)"]
pub fn drop(&self, row: i32, orientation: i32, data_transfer: Option<&DataTransfer>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getLevel()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getLevel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.getLevel($1)"]
pub fn get_level(&self, row: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getParentIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getParentIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.getParentIndex($1)"]
pub fn get_parent_index(&self, row: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRowProperties()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/getRowProperties)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.getRowProperties($1)"]
pub fn get_row_properties(&self, row: i32) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasNextSibling()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/hasNextSibling)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.hasNextSibling($1, $2)"]
pub fn has_next_sibling(&self, row: i32, after_index: i32) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `isContainer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.isContainer($1)"]
pub fn is_container(&self, row: i32) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `isContainerEmpty()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerEmpty)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.isContainerEmpty($1)"]
pub fn is_container_empty(&self, row: i32) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `isContainerOpen()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isContainerOpen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.isContainerOpen($1)"]
pub fn is_container_open(&self, row: i32) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `isSeparator()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSeparator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.isSeparator($1)"]
pub fn is_separator(&self, row: i32) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `isSorted()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/isSorted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.isSorted()"]
pub fn is_sorted(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `performAction()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.performAction($1)"]
pub fn perform_action(&self, action: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `performActionOnRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/performActionOnRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.performActionOnRow($1, $2)"]
pub fn perform_action_on_row(&self, action: &str, row: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `selectionChanged()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/selectionChanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.selectionChanged()"]
pub fn selection_changed(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TreeBoxObject")]
    #[doc = "The `setTree()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/setTree)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeView`*"]
impl TreeView {
    #[js::import = "$0.setTree($1)"]
pub fn set_tree(&self, tree: Option<&TreeBoxObject>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `toggleOpenState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeView/toggleOpenState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
impl TreeView {
    #[js::import = "$0.toggleOpenState($1)"]
pub fn toggle_open_state(&self, row: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl TreeView {
    #[doc = "The `TreeView.DROP_BEFORE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub const DROP_BEFORE: i16 = -1i64 as i16;
    #[doc = "The `TreeView.DROP_ON` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub const DROP_ON: i16 = 0i64 as i16;
    #[doc = "The `TreeView.DROP_AFTER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeView`*"]
    pub const DROP_AFTER: i16 = 1u64 as i16;
}
