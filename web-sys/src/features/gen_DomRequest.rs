#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomRequest(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomRequestReadyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`, `DomRequestReadyState`*"]
impl DomRequest {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> DomRequestReadyState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
impl DomRequest {
    #[js::import = "$0.result"]
pub fn result(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomException")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `DomRequest`*"]
impl DomRequest {
    #[js::import = "$0.error"]
pub fn error(&self) -> Option<DomException> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onsuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
impl DomRequest {
    #[js::import = "$0.onsuccess"]
pub fn onsuccess(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onsuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
impl DomRequest {
    #[js::import = "$0.onsuccess = $1"]
pub fn set_onsuccess(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
impl DomRequest {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
impl DomRequest {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `then()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
impl DomRequest {
    #[js::import = "$0.then()"]
pub fn then(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `then()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
impl DomRequest {
    #[js::import = "$0.then($1)"]
pub fn then_with_fulfill_callback(&self, fulfill_callback: Option<&::js_sys::Function>) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `then()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRequest/then)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`*"]
impl DomRequest {
    #[js::import = "$0.then($1, $2)"]
pub fn then_with_fulfill_callback_and_reject_callback(&self, fulfill_callback: Option<&::js_sys::Function>, reject_callback: Option<&::js_sys::Function>) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
