#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlOutputElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlOutputElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomTokenList")]
    #[doc = "Getter for the `htmlFor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/htmlFor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomTokenList`, `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.htmlFor"]
pub fn html_for(&self) -> DomTokenList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.form"]
pub fn form(&self) -> Option<HtmlFormElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `defaultValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.defaultValue"]
pub fn default_value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `defaultValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/defaultValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.defaultValue = $1"]
pub fn set_default_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.value"]
pub fn value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.willValidate"]
pub fn will_validate(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`, `ValidityState`*"]
impl HtmlOutputElement {
    #[js::import = "$0.validity"]
pub fn validity(&self) -> ValidityState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.validationMessage"]
pub fn validation_message(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`, `NodeList`*"]
impl HtmlOutputElement {
    #[js::import = "$0.labels"]
pub fn labels(&self) -> NodeList { panic!("web-sys intrinsic") }
}
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.checkValidity()"]
pub fn check_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.reportValidity()"]
pub fn report_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLOutputElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlOutputElement`*"]
impl HtmlOutputElement {
    #[js::import = "$0.setCustomValidity($1)"]
pub fn set_custom_validity(&self, error: &str) { panic!("web-sys intrinsic") }
}
