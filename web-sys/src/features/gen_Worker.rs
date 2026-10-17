#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Worker` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Worker(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "$0.onmessageerror"]
pub fn onmessageerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "$0.onmessageerror = $1"]
pub fn set_onmessageerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Worker(..)` constructor, creating a new instance of `Worker`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "new Worker($0)"]
pub fn new(script_url: &str) -> Result<Worker, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WorkerOptions")]
    #[doc = "The `new Worker(..)` constructor, creating a new instance of `Worker`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/Worker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`, `WorkerOptions`*"]
impl Worker {
    #[js::import = "new Worker($0, $1)"]
pub fn new_with_options(script_url: &str, options: &WorkerOptions) -> Result<Worker, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "$0.postMessage($1)"]
pub fn post_message(&self, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "$0.postMessage($1, $2)"]
pub fn post_message_with_transfer(&self, message: &::wasm_bindgen::JsValue, transfer: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `terminate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Worker/terminate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Worker`*"]
impl Worker {
    #[js::import = "$0.terminate()"]
pub fn terminate(&self) { panic!("web-sys intrinsic") }
}
