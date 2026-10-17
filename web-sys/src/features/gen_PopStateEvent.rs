#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopStateEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PopStateEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEvent`*"]
impl PopStateEvent {
    #[js::import = "$0.state"]
pub fn state(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "The `new PopStateEvent(..)` constructor, creating a new instance of `PopStateEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/PopStateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEvent`*"]
impl PopStateEvent {
    #[js::import = "new PopStateEvent($0)"]
pub fn new(type_: &str) -> Result<PopStateEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PopStateEventInit")]
    #[doc = "The `new PopStateEvent(..)` constructor, creating a new instance of `PopStateEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/PopStateEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEvent`, `PopStateEventInit`*"]
impl PopStateEvent {
    #[js::import = "new PopStateEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &PopStateEventInit,) -> Result<PopStateEvent, JsValue> { panic!("web-sys intrinsic") }
}
