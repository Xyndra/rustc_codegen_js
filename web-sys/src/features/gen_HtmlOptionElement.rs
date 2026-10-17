#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOptionElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlOptionElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.disabled"]
pub fn disabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.disabled = $1"]
pub fn set_disabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.form"]
pub fn form(&self) -> Option<HtmlFormElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.label = $1"]
pub fn set_label(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `defaultSelected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.defaultSelected"]
pub fn default_selected(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `defaultSelected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/defaultSelected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.defaultSelected = $1"]
pub fn set_default_selected(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `selected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.selected"]
pub fn selected(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `selected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/selected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.selected = $1"]
pub fn set_selected(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.value"]
pub fn value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.text"]
pub fn text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `text` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.text = $1"]
pub fn set_text(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `index` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/index)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl HtmlOptionElement {
    #[js::import = "$0.index"]
pub fn index(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl Option {
    #[js::import = "new Option()"]
pub fn new() -> Result<HtmlOptionElement, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl Option {
    #[js::import = "new Option($0)"]
pub fn new_with_text(text: &str) -> Result<HtmlOptionElement, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl Option {
    #[js::import = "new Option($0, $1)"]
pub fn new_with_text_and_value(text: &str, value: &str) -> Result<HtmlOptionElement, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl Option {
    #[js::import = "new Option($0, $1, $2)"]
pub fn new_with_text_and_value_and_default_selected(text: &str, value: &str, default_selected: bool,) -> Result<HtmlOptionElement, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new HtmlOptionElement(..)` constructor, creating a new instance of `HtmlOptionElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOptionElement/HTMLOptionElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOptionElement`*"]
impl Option {
    #[js::import = "new Option($0, $1, $2, $3)"]
pub fn new_with_text_and_value_and_default_selected_and_selected(text: &str, value: &str, default_selected: bool, selected: bool,) -> Result<HtmlOptionElement, JsValue> { panic!("web-sys intrinsic") }
}
