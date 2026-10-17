#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventTarget` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
#[js::opaque]
#[derive(Copy)]
pub struct EventTarget(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new EventTarget(..)` constructor, creating a new instance of `EventTarget`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/EventTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
impl EventTarget {
    #[js::import = "new EventTarget()"]
pub fn new() -> Result<EventTarget, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2)"]
pub fn add_event_listener_with_callback(&self, type_: &str, listener: &::wasm_bindgen::JsValue) -> Result<(), JsValue> {
    panic!("web-sys intrinsic")
}
}
    #[cfg(feature = "EventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2)"]
pub fn add_event_listener_with_event_listener(&self, type_: &str, listener: &EventListener) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AddEventListenerOptions")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2, $3)"]
pub fn add_event_listener_with_callback_and_add_event_listener_options(&self, type_: &str, listener: &::js_sys::Function, options: &AddEventListenerOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "AddEventListenerOptions", feature = "EventListener",))]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventListener`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2, $3)"]
pub fn add_event_listener_with_event_listener_and_add_event_listener_options(&self, type_: &str, listener: &EventListener, options: &AddEventListenerOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2, $3)"]
pub fn add_event_listener_with_callback_and_bool(&self, type_: &str, listener: &::js_sys::Function, options: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2, $3)"]
pub fn add_event_listener_with_event_listener_and_bool(&self, type_: &str, listener: &EventListener, options: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AddEventListenerOptions")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2, $3, $4)"]
pub fn add_event_listener_with_callback_and_add_event_listener_options_and_wants_untrusted(&self, type_: &str, listener: &::js_sys::Function, options: &AddEventListenerOptions, wants_untrusted: Option<bool>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "AddEventListenerOptions", feature = "EventListener",))]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`, `EventListener`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2, $3, $4)"]
pub fn add_event_listener_with_event_listener_and_add_event_listener_options_and_wants_untrusted(&self, type_: &str, listener: &EventListener, options: &AddEventListenerOptions, wants_untrusted: Option<bool>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2, $3, $4)"]
pub fn add_event_listener_with_callback_and_bool_and_wants_untrusted(&self, type_: &str, listener: &::js_sys::Function, options: bool, wants_untrusted: Option<bool>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventListener")]
    #[doc = "The `addEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.addEventListener($1, $2, $3, $4)"]
pub fn add_event_listener_with_event_listener_and_bool_and_wants_untrusted(&self, type_: &str, listener: &EventListener, options: bool, wants_untrusted: Option<bool>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Event")]
    #[doc = "The `dispatchEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.dispatchEvent($1)"]
pub fn dispatch_event(&self, event: &Event) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.removeEventListener($1, $2)"]
pub fn remove_event_listener_with_callback(&self, type_: &str, listener: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventListener")]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.removeEventListener($1, $2)"]
pub fn remove_event_listener_with_event_listener(&self, type_: &str, listener: &EventListener) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventListenerOptions")]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.removeEventListener($1, $2, $3)"]
pub fn remove_event_listener_with_callback_and_event_listener_options(&self, type_: &str, listener: &::js_sys::Function, options: &EventListenerOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "EventListener", feature = "EventListenerOptions",))]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventListenerOptions`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.removeEventListener($1, $2, $3)"]
pub fn remove_event_listener_with_event_listener_and_event_listener_options(&self, type_: &str, listener: &EventListener, options: &EventListenerOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.removeEventListener($1, $2, $3)"]
pub fn remove_event_listener_with_callback_and_bool(&self, type_: &str, listener: &::js_sys::Function, options: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventListener")]
    #[doc = "The `removeEventListener()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListener`, `EventTarget`*"]
impl EventTarget {
    #[js::import = "$0.removeEventListener($1, $2, $3)"]
pub fn remove_event_listener_with_event_listener_and_bool(&self, type_: &str, listener: &EventListener, options: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
