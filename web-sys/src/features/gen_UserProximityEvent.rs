#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UserProximityEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct UserProximityEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `near` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent/near)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEvent`*"]
impl UserProximityEvent {
    #[js::import = "$0.near"]
pub fn near(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `new UserProximityEvent(..)` constructor, creating a new instance of `UserProximityEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent/UserProximityEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEvent`*"]
impl UserProximityEvent {
    #[js::import = "new UserProximityEvent($0)"]
pub fn new(type_: &str) -> Result<UserProximityEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "UserProximityEventInit")]
    #[doc = "The `new UserProximityEvent(..)` constructor, creating a new instance of `UserProximityEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserProximityEvent/UserProximityEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEvent`, `UserProximityEventInit`*"]
impl UserProximityEvent {
    #[js::import = "new UserProximityEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &UserProximityEventInit,) -> Result<UserProximityEvent, JsValue> { panic!("web-sys intrinsic") }
}
