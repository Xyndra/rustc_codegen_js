#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTableElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlTableElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlTableCaptionElement")]
    #[doc = "Getter for the `caption` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.caption"]
pub fn caption(&self) -> Option<HtmlTableCaptionElement> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlTableCaptionElement")]
    #[doc = "Setter for the `caption` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/caption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableCaptionElement`, `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.caption = $1"]
pub fn set_caption(&self, value: Option<&HtmlTableCaptionElement>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlTableSectionElement")]
    #[doc = "Getter for the `tHead` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.tHead"]
pub fn t_head(&self) -> Option<HtmlTableSectionElement> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlTableSectionElement")]
    #[doc = "Setter for the `tHead` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tHead)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.tHead = $1"]
pub fn set_t_head(&self, value: Option<&HtmlTableSectionElement>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlTableSectionElement")]
    #[doc = "Getter for the `tFoot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.tFoot"]
pub fn t_foot(&self) -> Option<HtmlTableSectionElement> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlTableSectionElement")]
    #[doc = "Setter for the `tFoot` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tFoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`, `HtmlTableSectionElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.tFoot = $1"]
pub fn set_t_foot(&self, value: Option<&HtmlTableSectionElement>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `tBodies` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/tBodies)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.tBodies"]
pub fn t_bodies(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `rows` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.rows"]
pub fn rows(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.align"]
pub fn align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.align = $1"]
pub fn set_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `border` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.border"]
pub fn border(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `border` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/border)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.border = $1"]
pub fn set_border(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `frame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.frame"]
pub fn frame(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `frame` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/frame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.frame = $1"]
pub fn set_frame(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.rules"]
pub fn rules(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `rules` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/rules)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.rules = $1"]
pub fn set_rules(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `summary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.summary"]
pub fn summary(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `summary` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/summary)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.summary = $1"]
pub fn set_summary(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.width = $1"]
pub fn set_width(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.bgColor"]
pub fn bg_color(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.bgColor = $1"]
pub fn set_bg_color(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `cellPadding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.cellPadding"]
pub fn cell_padding(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `cellPadding` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellPadding)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.cellPadding = $1"]
pub fn set_cell_padding(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `cellSpacing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.cellSpacing"]
pub fn cell_spacing(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `cellSpacing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/cellSpacing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.cellSpacing = $1"]
pub fn set_cell_spacing(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `createCaption()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createCaption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.createCaption()"]
pub fn create_caption(&self) -> HtmlElement { panic!("web-sys intrinsic") }
}
    #[doc = "The `createTBody()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTBody)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.createTBody()"]
pub fn create_t_body(&self) -> HtmlElement { panic!("web-sys intrinsic") }
}
    #[doc = "The `createTFoot()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTFoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.createTFoot()"]
pub fn create_t_foot(&self) -> HtmlElement { panic!("web-sys intrinsic") }
}
    #[doc = "The `createTHead()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/createTHead)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.createTHead()"]
pub fn create_t_head(&self) -> HtmlElement { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteCaption()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteCaption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.deleteCaption()"]
pub fn delete_caption(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.deleteRow($1)"]
pub fn delete_row(&self, index: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteTFoot()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTFoot)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.deleteTFoot()"]
pub fn delete_t_foot(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteTHead()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/deleteTHead)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.deleteTHead()"]
pub fn delete_t_head(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.insertRow()"]
pub fn insert_row(&self) -> Result<HtmlElement, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `insertRow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTableElement/insertRow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTableElement`*"]
impl HtmlTableElement {
    #[js::import = "$0.insertRow($1)"]
pub fn insert_row_with_index(&self, index: i32) -> Result<HtmlElement, JsValue> { panic!("web-sys intrinsic") }
}
