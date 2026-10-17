#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DedicatedWorkerGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DedicatedWorkerGlobalScope(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.onmessageerror"]
pub fn onmessageerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.onmessageerror = $1"]
pub fn set_onmessageerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onrtctransform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onrtctransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.onrtctransform"]
pub fn onrtctransform(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onrtctransform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/onrtctransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.onrtctransform = $1"]
pub fn set_onrtctransform(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.postMessage($1)"]
pub fn post_message(&self, message: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.postMessage($1, $2)"]
pub fn post_message_with_transfer(&self, message: &::wasm_bindgen::JsValue, transfer: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancelAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/cancelAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.cancelAnimationFrame($1)"]
pub fn cancel_animation_frame(&self, handle: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/requestAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DedicatedWorkerGlobalScope`*"]
impl DedicatedWorkerGlobalScope {
    #[js::import = "$0.requestAnimationFrame($1)"]
pub fn request_animation_frame(&self, callback: &::js_sys::Function) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
