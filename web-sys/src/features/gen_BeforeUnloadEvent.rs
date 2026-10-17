#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BeforeUnloadEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BeforeUnloadEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `returnValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
impl BeforeUnloadEvent {
    #[js::import = "$0.returnValue"]
pub fn return_value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `returnValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BeforeUnloadEvent/returnValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BeforeUnloadEvent`*"]
impl BeforeUnloadEvent {
    #[js::import = "$0.returnValue = $1"]
pub fn set_return_value(&self, value: &str) { panic!("web-sys intrinsic") }
}
