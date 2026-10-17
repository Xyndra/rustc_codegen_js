#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorker` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ServiceWorker(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `scriptURL` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/scriptURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
impl ServiceWorker {
    #[js::import = "$0.scriptURL"]
pub fn script_url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServiceWorkerState")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerState`*"]
impl ServiceWorker {
    #[js::import = "$0.state"]
pub fn state(&self) -> ServiceWorkerState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
impl ServiceWorker {
    #[js::import = "$0.onstatechange"]
pub fn onstatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onstatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
impl ServiceWorker {
    #[js::import = "$0.onstatechange = $1"]
pub fn set_onstatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
impl ServiceWorker {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
impl ServiceWorker {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
impl ServiceWorker {
    #[js::import = "$0.postMessage($1)"]
pub fn post_message(&self, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`*"]
impl ServiceWorker {
    #[js::import = "$0.postMessage($1, $2)"]
pub fn post_message_with_transferable(&self, message: &::wasm_bindgen::JsValue, transferable: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
