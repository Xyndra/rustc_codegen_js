#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PresentationRequest(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
impl PresentationRequest {
    #[js::import = "$0.onconnectionavailable"]
pub fn onconnectionavailable(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
impl PresentationRequest {
    #[js::import = "$0.onconnectionavailable = $1"]
pub fn set_onconnectionavailable(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
impl PresentationRequest {
    #[js::import = "new PresentationRequest($0)"]
pub fn new_with_url(url: &str) -> Result<PresentationRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new PresentationRequest(..)` constructor, creating a new instance of `PresentationRequest`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/PresentationRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
impl PresentationRequest {
    #[js::import = "new PresentationRequest($0)"]
pub fn new_with_urls(urls: &::wasm_bindgen::JsValue) -> Result<PresentationRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAvailability()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/getAvailability)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
impl PresentationRequest {
    #[js::import = "$0.getAvailability()"]
pub fn get_availability(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `reconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/reconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
impl PresentationRequest {
    #[js::import = "$0.reconnect($1)"]
pub fn reconnect(&self, presentation_id: &str) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationRequest/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationRequest`*"]
impl PresentationRequest {
    #[js::import = "$0.start()"]
pub fn start(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
