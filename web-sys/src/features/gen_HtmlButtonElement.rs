#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlButtonElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlButtonElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
impl HtmlButtonElement {
    #[js::import = "$0.autofocus"]
pub fn autofocus(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `autofocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/autofocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
    #[deprecated(
        note = "Use `HtmlElement::autofocus()` or `HtmlElement::set_autofocus()` instead."
    )]
impl HtmlButtonElement {
    #[js::import = "$0.autofocus = $1"]
pub fn set_autofocus(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.disabled"]
pub fn disabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.disabled = $1"]
pub fn set_disabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`, `HtmlFormElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.form"]
pub fn form(&self) -> Option<HtmlFormElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `formAction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formAction"]
pub fn form_action(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `formAction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formAction = $1"]
pub fn set_form_action(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `formEnctype` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formEnctype"]
pub fn form_enctype(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `formEnctype` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formEnctype)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formEnctype = $1"]
pub fn set_form_enctype(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `formMethod` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formMethod"]
pub fn form_method(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `formMethod` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formMethod)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formMethod = $1"]
pub fn set_form_method(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `formNoValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formNoValidate"]
pub fn form_no_validate(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `formNoValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formNoValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formNoValidate = $1"]
pub fn set_form_no_validate(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `formTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formTarget"]
pub fn form_target(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `formTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/formTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.formTarget = $1"]
pub fn set_form_target(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.value"]
pub fn value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.value = $1"]
pub fn set_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `willValidate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/willValidate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.willValidate"]
pub fn will_validate(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ValidityState")]
    #[doc = "Getter for the `validity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`, `ValidityState`*"]
impl HtmlButtonElement {
    #[js::import = "$0.validity"]
pub fn validity(&self) -> ValidityState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `validationMessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/validationMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.validationMessage"]
pub fn validation_message(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NodeList")]
    #[doc = "Getter for the `labels` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/labels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`, `NodeList`*"]
impl HtmlButtonElement {
    #[js::import = "$0.labels"]
pub fn labels(&self) -> NodeList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `popoverTargetElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.popoverTargetElement"]
pub fn popover_target_element(&self) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `popoverTargetElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.popoverTargetElement = $1"]
pub fn set_popover_target_element(&self, value: Option<&Element>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `popoverTargetAction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.popoverTargetAction"]
pub fn popover_target_action(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `popoverTargetAction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/popoverTargetAction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.popoverTargetAction = $1"]
pub fn set_popover_target_action(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `checkValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/checkValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.checkValidity()"]
pub fn check_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `reportValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/reportValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.reportValidity()"]
pub fn report_validity(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCustomValidity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLButtonElement/setCustomValidity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlButtonElement`*"]
impl HtmlButtonElement {
    #[js::import = "$0.setCustomValidity($1)"]
pub fn set_custom_validity(&self, error: &str) { panic!("web-sys intrinsic") }
}
