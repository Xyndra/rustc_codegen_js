#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ToggleEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ToggleEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `oldState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/oldState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`*"]
impl ToggleEvent {
    #[js::import = "$0.oldState"]
pub fn old_state(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `newState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/newState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`*"]
impl ToggleEvent {
    #[js::import = "$0.newState"]
pub fn new_state(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ToggleEvent(..)` constructor, creating a new instance of `ToggleEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/ToggleEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`*"]
impl ToggleEvent {
    #[js::import = "new ToggleEvent($0)"]
pub fn new(type_: &str) -> Result<ToggleEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ToggleEventInit")]
    #[doc = "The `new ToggleEvent(..)` constructor, creating a new instance of `ToggleEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ToggleEvent/ToggleEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEvent`, `ToggleEventInit`*"]
impl ToggleEvent {
    #[js::import = "new ToggleEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &ToggleEventInit,) -> Result<ToggleEvent, JsValue> { panic!("web-sys intrinsic") }
}
