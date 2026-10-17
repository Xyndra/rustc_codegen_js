#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SharedWorkerGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SharedWorkerGlobalScope(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
impl SharedWorkerGlobalScope {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
impl SharedWorkerGlobalScope {
    #[js::import = "$0.onconnect"]
pub fn onconnect(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/onconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
impl SharedWorkerGlobalScope {
    #[js::import = "$0.onconnect = $1"]
pub fn set_onconnect(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SharedWorkerGlobalScope/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SharedWorkerGlobalScope`*"]
impl SharedWorkerGlobalScope {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
