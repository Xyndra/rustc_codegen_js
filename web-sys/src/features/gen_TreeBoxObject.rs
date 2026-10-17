#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeBoxObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TreeBoxObject(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `focused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.focused"]
pub fn focused(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `focused` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/focused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.focused = $1"]
pub fn set_focused(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `treeBody` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/treeBody)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.treeBody"]
pub fn tree_body(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rowHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.rowHeight"]
pub fn row_height(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rowWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.rowWidth"]
pub fn row_width(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `horizontalPosition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/horizontalPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.horizontalPosition"]
pub fn horizontal_position(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `beginUpdateBatch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/beginUpdateBatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.beginUpdateBatch()"]
pub fn begin_update_batch(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearStyleAndImageCaches()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/clearStyleAndImageCaches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.clearStyleAndImageCaches()"]
pub fn clear_style_and_image_caches(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `endUpdateBatch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/endUpdateBatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.endUpdateBatch()"]
pub fn end_update_batch(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `ensureRowIsVisible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/ensureRowIsVisible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.ensureRowIsVisible($1)"]
pub fn ensure_row_is_visible(&self, index: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TreeCellInfo")]
    #[doc = "The `getCellAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`, `TreeCellInfo`*"]
impl TreeBoxObject {
    #[js::import = "$0.getCellAt($1, $2)"]
pub fn get_cell_at(&self, x: i32, y: i32) -> Result<TreeCellInfo, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getCellAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getCellAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.getCellAt($1, $2, $3, $4, $5)"]
pub fn get_cell_at_with_row_and_column_and_child_elt(&self, x: i32, y: i32, row: &::js_sys::Object, column: &::js_sys::Object, child_elt: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFirstVisibleRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getFirstVisibleRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.getFirstVisibleRow()"]
pub fn get_first_visible_row(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getLastVisibleRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getLastVisibleRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.getLastVisibleRow()"]
pub fn get_last_visible_row(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getPageLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getPageLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.getPageLength()"]
pub fn get_page_length(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRowAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/getRowAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.getRowAt($1, $2)"]
pub fn get_row_at(&self, x: i32, y: i32) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `invalidate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.invalidate()"]
pub fn invalidate(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `invalidateRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.invalidateRange($1, $2)"]
pub fn invalidate_range(&self, start_index: i32, end_index: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `invalidateRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/invalidateRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.invalidateRow($1)"]
pub fn invalidate_row(&self, index: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `rowCountChanged()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/rowCountChanged)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.rowCountChanged($1, $2)"]
pub fn row_count_changed(&self, index: i32, count: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollByLines()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByLines)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.scrollByLines($1)"]
pub fn scroll_by_lines(&self, num_lines: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollByPages()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollByPages)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.scrollByPages($1)"]
pub fn scroll_by_pages(&self, num_pages: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollToRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TreeBoxObject/scrollToRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeBoxObject`*"]
impl TreeBoxObject {
    #[js::import = "$0.scrollToRow($1)"]
pub fn scroll_to_row(&self, index: i32) { panic!("web-sys intrinsic") }
}
