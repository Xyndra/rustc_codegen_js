#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BroadcastChannel` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BroadcastChannel(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
impl BroadcastChannel {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
impl BroadcastChannel {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
impl BroadcastChannel {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
impl BroadcastChannel {
    #[js::import = "$0.onmessageerror"]
pub fn onmessageerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
impl BroadcastChannel {
    #[js::import = "$0.onmessageerror = $1"]
pub fn set_onmessageerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new BroadcastChannel(..)` constructor, creating a new instance of `BroadcastChannel`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/BroadcastChannel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
impl BroadcastChannel {
    #[js::import = "new BroadcastChannel($0)"]
pub fn new(channel: &str) -> Result<BroadcastChannel, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
impl BroadcastChannel {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BroadcastChannel`*"]
impl BroadcastChannel {
    #[js::import = "$0.postMessage($1)"]
pub fn post_message(&self, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
