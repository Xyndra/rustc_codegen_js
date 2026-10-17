#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerDebuggerGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WorkerDebuggerGlobalScope(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `global` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/global)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.global"]
pub fn global(&self) -> Result<::js_sys::Object, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `createSandbox()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/createSandbox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.createSandbox($1, $2)"]
pub fn create_sandbox(&self, name: &str, prototype: &::js_sys::Object) -> Result<::js_sys::Object, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `dump()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/dump)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.dump()"]
pub fn dump(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `dump()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/dump)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.dump($1)"]
pub fn dump_with_string(&self, string: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `enterEventLoop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/enterEventLoop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.enterEventLoop()"]
pub fn enter_event_loop(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `leaveEventLoop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/leaveEventLoop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.leaveEventLoop()"]
pub fn leave_event_loop(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `loadSubScript()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/loadSubScript)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.loadSubScript($1)"]
pub fn load_sub_script(&self, url: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `loadSubScript()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/loadSubScript)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.loadSubScript($1, $2)"]
pub fn load_sub_script_with_sandbox(&self, url: &str, sandbox: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.postMessage($1)"]
pub fn post_message(&self, message: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `reportError()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/reportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.reportError($1)"]
pub fn report_error(&self, message: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `retrieveConsoleEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/retrieveConsoleEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.retrieveConsoleEvents()"]
pub fn retrieve_console_events(&self) -> Result<::js_sys::Array, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setConsoleEventHandler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/setConsoleEventHandler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.setConsoleEventHandler($1)"]
pub fn set_console_event_handler(&self, handler: Option<&::js_sys::Function>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setImmediate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerDebuggerGlobalScope/setImmediate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerDebuggerGlobalScope`*"]
impl WorkerDebuggerGlobalScope {
    #[js::import = "$0.setImmediate($1)"]
pub fn set_immediate(&self, handler: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
