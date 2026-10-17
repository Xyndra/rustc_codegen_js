#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDialogElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlDialogElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `open` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
impl HtmlDialogElement {
    #[js::import = "$0.open"]
pub fn open(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `open` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
impl HtmlDialogElement {
    #[js::import = "$0.open = $1"]
pub fn set_open(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `returnValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
impl HtmlDialogElement {
    #[js::import = "$0.returnValue"]
pub fn return_value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `returnValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/returnValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
impl HtmlDialogElement {
    #[js::import = "$0.returnValue = $1"]
pub fn set_return_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
impl HtmlDialogElement {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
impl HtmlDialogElement {
    #[js::import = "$0.close($1)"]
pub fn close_with_return_value(&self, return_value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `show()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/show)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
impl HtmlDialogElement {
    #[js::import = "$0.show()"]
pub fn show(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `showModal()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDialogElement/showModal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDialogElement`*"]
impl HtmlDialogElement {
    #[js::import = "$0.showModal()"]
pub fn show_modal(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
