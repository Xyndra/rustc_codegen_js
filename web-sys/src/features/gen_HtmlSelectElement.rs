#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlSelectElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlSelectElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
impl HtmlSelectElement {
    #[js::import = "$0.autofocus"]
pub fn autofocus(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
impl HtmlSelectElement {
    #[js::import = "$0.autofocus = $1"]
pub fn set_autofocus(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `autocomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.autocomplete"]
pub fn autocomplete(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `autocomplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/autocomplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.autocomplete = $1"]
pub fn set_autocomplete(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.disabled"]
pub fn disabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.disabled = $1"]
pub fn set_disabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.form"]
pub fn form(&self) -> Option<HtmlFormElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `multiple` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.multiple"]
pub fn multiple(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `multiple` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/multiple)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.multiple = $1"]
pub fn set_multiple(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `required` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.required"]
pub fn required(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `required` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/required)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.required = $1"]
pub fn set_required(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.size"]
pub fn size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.size = $1"]
pub fn set_size(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptionsCollection")]
    #[doc = "Getter for the `options` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/options)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionsCollection`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.options"]
pub fn options(&self) -> HtmlOptionsCollection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.length = $1"]
pub fn set_length(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `selectedOptions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedOptions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.selectedOptions"]
pub fn selected_options(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.selectedIndex"]
pub fn selected_index(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `selectedIndex` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/selectedIndex)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.selectedIndex = $1"]
pub fn set_selected_index(&self, value: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.value"]
pub fn value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.willValidate"]
pub fn will_validate(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`, `ValidityState`*"]
impl HtmlSelectElement {
    #[js::import = "$0.validity"]
pub fn validity(&self) -> ValidityState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.validationMessage"]
pub fn validation_message(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`, `NodeList`*"]
impl HtmlSelectElement {
    #[js::import = "$0.labels"]
pub fn labels(&self) -> NodeList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptionElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.add($1)"]
pub fn add_with_html_option_element(&self, element: &HtmlOptionElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptGroupElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.add($1)"]
pub fn add_with_html_opt_group_element(&self, element: &HtmlOptGroupElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptionElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_html_option_element_and_opt_html_element(&self, element: &HtmlOptionElement, before: Option<&HtmlElement>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptGroupElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_html_opt_group_element_and_opt_html_element(&self, element: &HtmlOptGroupElement, before: Option<&HtmlElement>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptionElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_html_option_element_and_opt_i32(&self, element: &HtmlOptionElement, before: Option<i32>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptGroupElement")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptGroupElement`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.add($1, $2)"]
pub fn add_with_html_opt_group_element_and_opt_i32(&self, element: &HtmlOptGroupElement, before: Option<i32>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.checkValidity()"]
pub fn check_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptionElement")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.namedItem($1)"]
pub fn named_item(&self, name: &str) -> Option<HtmlOptionElement> { panic!("web-sys intrinsic") }
}
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.remove($1)"]
pub fn remove_with_index(&self, index: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.remove()"]
pub fn remove(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.reportValidity()"]
pub fn report_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLSelectElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.setCustomValidity($1)"]
pub fn set_custom_validity(&self, error: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlSelectElement`*"]
impl HtmlSelectElement {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlOptionElement")]
        catch,
        method,
        structural,
        js_class = "HTMLSelectElement",
        indexing_setter
    )]
    #[doc = "Indexing setter. As in the literal Javascript `this[key] = value`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`, `HtmlSelectElement`*"]
#[js::import = "None"]
pub fn set(this: &HtmlSelectElement, index: u32, option: Option<&HtmlOptionElement>,) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
