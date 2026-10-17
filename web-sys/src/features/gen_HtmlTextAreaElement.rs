#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTextAreaElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlTextAreaElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `autocomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.autocomplete"]
pub fn autocomplete(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `autocomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autocomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.autocomplete = $1"]
pub fn set_autocomplete(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
impl HtmlTextAreaElement {
    #[js::import = "$0.autofocus"]
pub fn autofocus(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
impl HtmlTextAreaElement {
    #[js::import = "$0.autofocus = $1"]
pub fn set_autofocus(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `cols` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.cols"]
pub fn cols(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `cols` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/cols)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.cols = $1"]
pub fn set_cols(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.disabled"]
pub fn disabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.disabled = $1"]
pub fn set_disabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.form"]
pub fn form(&self) -> Option<HtmlFormElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.maxLength"]
pub fn max_length(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `maxLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/maxLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.maxLength = $1"]
pub fn set_max_length(&self, value: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `minLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.minLength"]
pub fn min_length(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `minLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/minLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.minLength = $1"]
pub fn set_min_length(&self, value: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `placeholder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.placeholder"]
pub fn placeholder(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `placeholder` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/placeholder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.placeholder = $1"]
pub fn set_placeholder(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `readOnly` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.readOnly"]
pub fn read_only(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `readOnly` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/readOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.readOnly = $1"]
pub fn set_read_only(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `required` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.required"]
pub fn required(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `required` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/required)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.required = $1"]
pub fn set_required(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rows` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.rows"]
pub fn rows(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `rows` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/rows)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.rows = $1"]
pub fn set_rows(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `wrap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.wrap"]
pub fn wrap(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `wrap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/wrap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.wrap = $1"]
pub fn set_wrap(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `defaultValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.defaultValue"]
pub fn default_value(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `defaultValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/defaultValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.defaultValue = $1"]
pub fn set_default_value(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.value"]
pub fn value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `textLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/textLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.textLength"]
pub fn text_length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.willValidate"]
pub fn will_validate(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`, `ValidityState`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.validity"]
pub fn validity(&self) -> ValidityState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.validationMessage"]
pub fn validation_message(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`, `NodeList`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.labels"]
pub fn labels(&self) -> NodeList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `selectionStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.selectionStart"]
pub fn selection_start(&self) -> Result<Option<u32>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `selectionStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.selectionStart = $1"]
pub fn set_selection_start(&self, value: Option<u32>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `selectionEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.selectionEnd"]
pub fn selection_end(&self) -> Result<Option<u32>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `selectionEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.selectionEnd = $1"]
pub fn set_selection_end(&self, value: Option<u32>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `selectionDirection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.selectionDirection"]
pub fn selection_direction(&self) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `selectionDirection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/selectionDirection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.selectionDirection = $1"]
pub fn set_selection_direction(&self, value: Option<&str>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.checkValidity()"]
pub fn check_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.reportValidity()"]
pub fn report_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `select()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/select)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.select()"]
pub fn select(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.setCustomValidity($1)"]
pub fn set_custom_validity(&self, error: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setRangeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.setRangeText($1)"]
pub fn set_range_text(&self, replacement: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setRangeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.setRangeText($1, $2, $3)"]
pub fn set_range_text_with_start_and_end(&self, replacement: &str, start: u32, end: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setRangeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setRangeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.setRangeText($1, $2, $3, $4)"]
pub fn set_range_text_with_start_and_end_and_mode(&self, replacement: &str, start: u32, end: u32, mode: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setSelectionRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.setSelectionRange($1, $2)"]
pub fn set_selection_range(&self, start: u32, end: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setSelectionRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTextAreaElement/setSelectionRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTextAreaElement`*"]
impl HtmlTextAreaElement {
    #[js::import = "$0.setSelectionRange($1, $2, $3)"]
pub fn set_selection_range_with_direction(&self, start: u32, end: u32, direction: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
