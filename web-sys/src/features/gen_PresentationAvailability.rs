#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationAvailability` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationAvailability`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PresentationAvailability(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationAvailability`*"]
impl PresentationAvailability {
    #[js::import = "$0.value"]
pub fn value(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationAvailability`*"]
impl PresentationAvailability {
    #[js::import = "$0.onchange"]
pub fn onchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationAvailability/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationAvailability`*"]
impl PresentationAvailability {
    #[js::import = "$0.onchange = $1"]
pub fn set_onchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
