#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaymentResponse` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PaymentResponse(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `requestId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/requestId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.requestId"]
pub fn request_id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `methodName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/methodName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.methodName"]
pub fn method_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `details` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/details)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.details"]
pub fn details(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PaymentAddress")]
    #[doc = "Getter for the `shippingAddress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingAddress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentAddress`, `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.shippingAddress"]
pub fn shipping_address(&self) -> Option<PaymentAddress> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `shippingOption` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/shippingOption)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.shippingOption"]
pub fn shipping_option(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `payerName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.payerName"]
pub fn payer_name(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `payerEmail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerEmail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.payerEmail"]
pub fn payer_email(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `payerPhone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/payerPhone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.payerPhone"]
pub fn payer_phone(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `complete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.complete()"]
pub fn complete(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PaymentComplete")]
    #[doc = "The `complete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/complete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentComplete`, `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.complete($1)"]
pub fn complete_with_result(&self, result: PaymentComplete) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaymentResponse/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaymentResponse`*"]
impl PaymentResponse {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
