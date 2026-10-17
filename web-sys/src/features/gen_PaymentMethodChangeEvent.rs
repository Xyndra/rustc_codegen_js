#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentMethodChangeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PaymentMethodChangeEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `methodName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
impl PaymentMethodChangeEvent {
    #[js::import = "$0.methodName"]
pub fn method_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `methodDetails` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/methodDetails)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
impl PaymentMethodChangeEvent {
    #[js::import = "$0.methodDetails"]
pub fn method_details(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`*"]
impl PaymentMethodChangeEvent {
    #[js::import = "new PaymentMethodChangeEvent($0)"]
pub fn new(type_: &str) -> Result<PaymentMethodChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PaymentMethodChangeEventInit")]
    #[doc = "The `new PaymentMethodChangeEvent(..)` constructor, creating a new instance of `PaymentMethodChangeEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentMethodChangeEvent/PaymentMethodChangeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentMethodChangeEvent`, `PaymentMethodChangeEventInit`*"]
impl PaymentMethodChangeEvent {
    #[js::import = "new PaymentMethodChangeEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &PaymentMethodChangeEventInit,) -> Result<PaymentMethodChangeEvent, JsValue> { panic!("web-sys intrinsic") }
}
