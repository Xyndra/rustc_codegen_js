#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessagePort` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MessagePort(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`*"]
impl MessagePort {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`*"]
impl MessagePort {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`*"]
impl MessagePort {
    #[js::import = "$0.onmessageerror"]
pub fn onmessageerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`*"]
impl MessagePort {
    #[js::import = "$0.onmessageerror = $1"]
pub fn set_onmessageerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`*"]
impl MessagePort {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`*"]
impl MessagePort {
    #[js::import = "$0.postMessage($1)"]
pub fn post_message(&self, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`*"]
impl MessagePort {
    #[js::import = "$0.postMessage($1, $2)"]
pub fn post_message_with_transferable(&self, message: &::wasm_bindgen::JsValue, transferable: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`*"]
impl MessagePort {
    #[js::import = "$0.start()"]
pub fn start(&self) { panic!("web-sys intrinsic") }
}
