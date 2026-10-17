#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SharedWorker` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorker`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SharedWorker(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MessagePort")]
    #[doc = "Getter for the `port` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/port)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessagePort`, `SharedWorker`*"]
impl SharedWorker {
    #[js::import = "$0.port"]
pub fn port(&self) -> MessagePort { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorker`*"]
impl SharedWorker {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorker`*"]
impl SharedWorker {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorker`*"]
impl SharedWorker {
    #[js::import = "new SharedWorker($0)"]
pub fn new(script_url: &str) -> Result<SharedWorker, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorker`*"]
impl SharedWorker {
    #[js::import = "new SharedWorker($0, $1)"]
pub fn new_with_str(script_url: &str, options: &str) -> Result<SharedWorker, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WorkerOptions")]
    #[doc = "The `new SharedWorker(..)` constructor, creating a new instance of `SharedWorker`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker/SharedWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorker`, `WorkerOptions`*"]
impl SharedWorker {
    #[js::import = "new SharedWorker($0, $1)"]
pub fn new_with_worker_options(script_url: &str, options: &WorkerOptions,) -> Result<SharedWorker, JsValue> { panic!("web-sys intrinsic") }
}
