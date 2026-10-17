#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlFieldSetElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlFieldSetElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.disabled"]
pub fn disabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.disabled = $1"]
pub fn set_disabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `HtmlFormElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.form"]
pub fn form(&self) -> Option<HtmlFormElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCollection")]
    #[doc = "Getter for the `elements` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/elements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCollection`, `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.elements"]
pub fn elements(&self) -> HtmlCollection { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.willValidate"]
pub fn will_validate(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`, `ValidityState`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.validity"]
pub fn validity(&self) -> ValidityState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.validationMessage"]
pub fn validation_message(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.checkValidity()"]
pub fn check_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.reportValidity()"]
pub fn report_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLFieldSetElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFieldSetElement`*"]
impl HtmlFieldSetElement {
    #[js::import = "$0.setCustomValidity($1)"]
pub fn set_custom_validity(&self, error: &str) { panic!("web-sys intrinsic") }
}
