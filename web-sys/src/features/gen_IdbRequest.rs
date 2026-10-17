#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbRequest(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
impl IdbRequest {
    #[js::import = "$0.result"]
pub fn result(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomException")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `IdbRequest`*"]
impl IdbRequest {
    #[js::import = "$0.error"]
pub fn error(&self) -> Result<Option<DomException>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
impl IdbRequest {
    #[js::import = "$0.source"]
pub fn source(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbTransaction")]
    #[doc = "Getter for the `transaction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/transaction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`, `IdbTransaction`*"]
impl IdbRequest {
    #[js::import = "$0.transaction"]
pub fn transaction(&self) -> Option<IdbTransaction> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbRequestReadyState")]
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`, `IdbRequestReadyState`*"]
impl IdbRequest {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> IdbRequestReadyState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
impl IdbRequest {
    #[js::import = "$0.onsuccess"]
pub fn onsuccess(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsuccess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onsuccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
impl IdbRequest {
    #[js::import = "$0.onsuccess = $1"]
pub fn set_onsuccess(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
impl IdbRequest {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBRequest/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbRequest`*"]
impl IdbRequest {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
