#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlLabelElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlLabelElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "Getter for the `form` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/form)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlFormElement`, `HtmlLabelElement`*"]
impl HtmlLabelElement {
    #[js::import = "$0.form"]
pub fn form(&self) -> Option<HtmlFormElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `htmlFor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
impl HtmlLabelElement {
    #[js::import = "$0.htmlFor"]
pub fn html_for(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `htmlFor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/htmlFor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
impl HtmlLabelElement {
    #[js::import = "$0.htmlFor = $1"]
pub fn set_html_for(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `control` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLLabelElement/control)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlLabelElement`*"]
impl HtmlLabelElement {
    #[js::import = "$0.control"]
pub fn control(&self) -> Option<HtmlElement> { panic!("web-sys intrinsic") }
}
