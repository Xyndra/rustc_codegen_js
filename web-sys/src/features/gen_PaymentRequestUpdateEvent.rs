#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentRequestUpdateEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PaymentRequestUpdateEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new PaymentRequestUpdateEvent(..)` constructor, creating a new instance of `PaymentRequestUpdateEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/PaymentRequestUpdateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`*"]
impl PaymentRequestUpdateEvent {
    #[js::import = "new PaymentRequestUpdateEvent($0)"]
pub fn new(type_: &str) -> Result<PaymentRequestUpdateEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PaymentRequestUpdateEventInit")]
    #[doc = "The `new PaymentRequestUpdateEvent(..)` constructor, creating a new instance of `PaymentRequestUpdateEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/PaymentRequestUpdateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`, `PaymentRequestUpdateEventInit`*"]
impl PaymentRequestUpdateEvent {
    #[js::import = "new PaymentRequestUpdateEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &PaymentRequestUpdateEventInit,) -> Result<PaymentRequestUpdateEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `updateWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentRequestUpdateEvent/updateWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentRequestUpdateEvent`*"]
impl PaymentRequestUpdateEvent {
    #[js::import = "$0.updateWith($1)"]
pub fn update_with(&self, details_promise: &::js_sys::Promise) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
