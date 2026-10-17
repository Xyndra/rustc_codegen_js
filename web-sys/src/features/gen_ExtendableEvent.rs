#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ExtendableEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new ExtendableEvent(..)` constructor, creating a new instance of `ExtendableEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/ExtendableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableEvent`*"]
impl ExtendableEvent {
    #[js::import = "new ExtendableEvent($0)"]
pub fn new(type_: &str) -> Result<ExtendableEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ExtendableEventInit")]
    #[doc = "The `new ExtendableEvent(..)` constructor, creating a new instance of `ExtendableEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/ExtendableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableEvent`, `ExtendableEventInit`*"]
impl ExtendableEvent {
    #[js::import = "new ExtendableEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &ExtendableEventInit,) -> Result<ExtendableEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `waitUntil()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableEvent/waitUntil)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableEvent`*"]
impl ExtendableEvent {
    #[js::import = "$0.waitUntil($1)"]
pub fn wait_until(&self, p: &::js_sys::Promise) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
