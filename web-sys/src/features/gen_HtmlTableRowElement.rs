#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableRowElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlTableRowElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `rowIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/rowIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.rowIndex"]
pub fn row_index(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sectionRowIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/sectionRowIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.sectionRowIndex"]
pub fn section_row_index(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `cells` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/cells)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.cells"]
pub fn cells(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.align"]
pub fn align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.align = $1"]
pub fn set_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.ch"]
pub fn ch(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.ch = $1"]
pub fn set_ch(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.chOff"]
pub fn ch_off(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.chOff = $1"]
pub fn set_ch_off(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.vAlign"]
pub fn v_align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.vAlign = $1"]
pub fn set_v_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.bgColor"]
pub fn bg_color(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.bgColor = $1"]
pub fn set_bg_color(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteCell()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/deleteCell)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.deleteCell($1)"]
pub fn delete_cell(&self, index: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertCell()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.insertCell()"]
pub fn insert_cell(&self) -> Result<HtmlElement, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertCell()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableRowElement/insertCell)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableRowElement`*"]
impl HtmlTableRowElement {
    #[js::import = "$0.insertCell($1)"]
pub fn insert_cell_with_index(&self, index: i32) -> Result<HtmlElement, JsValue> { panic!("web-sys intrinsic") }
}
