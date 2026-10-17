#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlObjectElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlObjectElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.data = $1"]
pub fn set_data(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `typeMustMatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.typeMustMatch"]
pub fn type_must_match(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `typeMustMatch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/typeMustMatch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.typeMustMatch = $1"]
pub fn set_type_must_match(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `useMap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.useMap"]
pub fn use_map(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `useMap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/useMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.useMap = $1"]
pub fn set_use_map(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.form"]
pub fn form(&self) -> Option<HtmlFormElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.width = $1"]
pub fn set_width(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.height = $1"]
pub fn set_height(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `contentDocument` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.contentDocument"]
pub fn content_document(&self) -> Option<Document> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `contentWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/contentWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`, `Window`*"]
impl HtmlObjectElement {
    #[js::import = "$0.contentWindow"]
pub fn content_window(&self) -> Option<Window> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.willValidate"]
pub fn will_validate(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`, `ValidityState`*"]
impl HtmlObjectElement {
    #[js::import = "$0.validity"]
pub fn validity(&self) -> ValidityState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.validationMessage"]
pub fn validation_message(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.align"]
pub fn align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `align` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/align)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.align = $1"]
pub fn set_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `archive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.archive"]
pub fn archive(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `archive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/archive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.archive = $1"]
pub fn set_archive(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.code"]
pub fn code(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.code = $1"]
pub fn set_code(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `declare` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.declare"]
pub fn declare(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `declare` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/declare)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.declare = $1"]
pub fn set_declare(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.hspace"]
pub fn hspace(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `hspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/hspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.hspace = $1"]
pub fn set_hspace(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `standby` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.standby"]
pub fn standby(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `standby` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/standby)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.standby = $1"]
pub fn set_standby(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `vspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.vspace"]
pub fn vspace(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `vspace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/vspace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.vspace = $1"]
pub fn set_vspace(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `codeBase` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.codeBase"]
pub fn code_base(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `codeBase` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeBase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.codeBase = $1"]
pub fn set_code_base(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `codeType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.codeType"]
pub fn code_type(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `codeType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/codeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.codeType = $1"]
pub fn set_code_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `border` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.border"]
pub fn border(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `border` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/border)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.border = $1"]
pub fn set_border(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.checkValidity()"]
pub fn check_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "The `getSVGDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/getSVGDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.getSVGDocument()"]
pub fn get_svg_document(&self) -> Option<Document> { panic!("web-sys intrinsic") }
}
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.reportValidity()"]
pub fn report_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLObjectElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlObjectElement`*"]
impl HtmlObjectElement {
    #[js::import = "$0.setCustomValidity($1)"]
pub fn set_custom_validity(&self, error: &str) { panic!("web-sys intrinsic") }
}
