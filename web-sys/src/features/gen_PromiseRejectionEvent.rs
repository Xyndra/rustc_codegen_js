#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PromiseRejectionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PromiseRejectionEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `promise` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/promise)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEvent`*"]
impl PromiseRejectionEvent {
    #[js::import = "$0.promise"]
pub fn promise(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEvent`*"]
impl PromiseRejectionEvent {
    #[js::import = "$0.reason"]
pub fn reason(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PromiseRejectionEventInit")]
    #[doc = "The `new PromiseRejectionEvent(..)` constructor, creating a new instance of `PromiseRejectionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent/PromiseRejectionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEvent`, `PromiseRejectionEventInit`*"]
impl PromiseRejectionEvent {
    #[js::import = "new PromiseRejectionEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &PromiseRejectionEventInit,) -> Result<PromiseRejectionEvent, JsValue> { panic!("web-sys intrinsic") }
}
