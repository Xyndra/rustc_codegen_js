#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ListBoxObject` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ListBoxObject(pub ::wasm_bindgen::JsValue);
    #[doc = "The `ensureIndexIsVisible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/ensureIndexIsVisible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
impl ListBoxObject {
    #[js::import = "$0.ensureIndexIsVisible($1)"]
pub fn ensure_index_is_visible(&self, row_index: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getIndexOfFirstVisibleRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfFirstVisibleRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
impl ListBoxObject {
    #[js::import = "$0.getIndexOfFirstVisibleRow()"]
pub fn get_index_of_first_visible_row(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `getIndexOfItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getIndexOfItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*"]
impl ListBoxObject {
    #[js::import = "$0.getIndexOfItem($1)"]
pub fn get_index_of_item(&self, item: &Element) -> i32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `getItemAtIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getItemAtIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `ListBoxObject`*"]
impl ListBoxObject {
    #[js::import = "$0.getItemAtIndex($1)"]
pub fn get_item_at_index(&self, index: i32) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getNumberOfVisibleRows()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getNumberOfVisibleRows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
impl ListBoxObject {
    #[js::import = "$0.getNumberOfVisibleRows()"]
pub fn get_number_of_visible_rows(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRowCount()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
impl ListBoxObject {
    #[js::import = "$0.getRowCount()"]
pub fn get_row_count(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRowHeight()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/getRowHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
impl ListBoxObject {
    #[js::import = "$0.getRowHeight()"]
pub fn get_row_height(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollByLines()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollByLines)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
impl ListBoxObject {
    #[js::import = "$0.scrollByLines($1)"]
pub fn scroll_by_lines(&self, num_lines: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollToIndex()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ListBoxObject/scrollToIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ListBoxObject`*"]
impl ListBoxObject {
    #[js::import = "$0.scrollToIndex($1)"]
pub fn scroll_to_index(&self, row_index: i32) { panic!("web-sys intrinsic") }
}
