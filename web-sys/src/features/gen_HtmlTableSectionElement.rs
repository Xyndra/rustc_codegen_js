#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableSectionElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlTableSectionElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `rows` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/rows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.rows"]
pub fn rows(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.align"]
pub fn align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.align = $1"]
pub fn set_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.ch"]
pub fn ch(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/ch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.ch = $1"]
pub fn set_ch(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.chOff"]
pub fn ch_off(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `chOff` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/chOff)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.chOff = $1"]
pub fn set_ch_off(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.vAlign"]
pub fn v_align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `vAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/vAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.vAlign = $1"]
pub fn set_v_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/deleteRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.deleteRow($1)"]
pub fn delete_row(&self, index: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.insertRow()"]
pub fn insert_row(&self) -> Result<HtmlElement, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableSectionElement/insertRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableSectionElement`*"]
impl HtmlTableSectionElement {
    #[js::import = "$0.insertRow($1)"]
pub fn insert_row_with_index(&self, index: i32) -> Result<HtmlElement, JsValue> { panic!("web-sys intrinsic") }
}
