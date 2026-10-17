#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NotificationEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NotificationEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Notification")]
    #[doc = "Getter for the `notification` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/notification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Notification`, `NotificationEvent`*"]
impl NotificationEvent {
    #[js::import = "$0.notification"]
pub fn notification(&self) -> Notification { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NotificationEventInit")]
    #[doc = "The `new NotificationEvent(..)` constructor, creating a new instance of `NotificationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NotificationEvent/NotificationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationEvent`, `NotificationEventInit`*"]
impl NotificationEvent {
    #[js::import = "new NotificationEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &NotificationEventInit,) -> Result<NotificationEvent, JsValue> { panic!("web-sys intrinsic") }
}
