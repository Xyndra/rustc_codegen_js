#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Event` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Event(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventTarget")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`, `EventTarget`*"]
impl Event {
    #[js::import = "$0.target"]
pub fn target(&self) -> Option<EventTarget> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventTarget")]
    #[doc = "Getter for the `currentTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/currentTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`, `EventTarget`*"]
impl Event {
    #[js::import = "$0.currentTarget"]
pub fn current_target(&self) -> Option<EventTarget> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `eventPhase` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/eventPhase)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.eventPhase"]
pub fn event_phase(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/bubbles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.bubbles"]
pub fn bubbles(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.cancelable"]
pub fn cancelable(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `defaultPrevented` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/defaultPrevented)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.defaultPrevented"]
pub fn default_prevented(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `composed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.composed"]
pub fn composed(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isTrusted` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/isTrusted)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.isTrusted"]
pub fn is_trusted(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `timeStamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/timeStamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.timeStamp"]
pub fn time_stamp(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `cancelBubble` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.cancelBubble"]
pub fn cancel_bubble(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `cancelBubble` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.cancelBubble = $1"]
pub fn set_cancel_bubble(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Event(..)` constructor, creating a new instance of `Event`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "new Event($0)"]
pub fn new(type_: &str) -> Result<Event, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventInit")]
    #[doc = "The `new Event(..)` constructor, creating a new instance of `Event`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/Event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`, `EventInit`*"]
impl Event {
    #[js::import = "new Event($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &EventInit,) -> Result<Event, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `composedPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/composedPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.composedPath()"]
pub fn composed_path(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `initEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.initEvent($1)"]
pub fn init_event(&self, type_: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.initEvent($1, $2)"]
pub fn init_event_with_bubbles(&self, type_: &str, bubbles: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.initEvent($1, $2, $3)"]
pub fn init_event_with_bubbles_and_cancelable(&self, type_: &str, bubbles: bool, cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `preventDefault()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.preventDefault()"]
pub fn prevent_default(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `stopImmediatePropagation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopImmediatePropagation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.stopImmediatePropagation()"]
pub fn stop_immediate_propagation(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `stopPropagation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
impl Event {
    #[js::import = "$0.stopPropagation()"]
pub fn stop_propagation(&self) { panic!("web-sys intrinsic") }
}
impl Event {
    #[doc = "The `Event.NONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub const NONE: u16 = 0i64 as u16;
    #[doc = "The `Event.CAPTURING_PHASE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub const CAPTURING_PHASE: u16 = 1u64 as u16;
    #[doc = "The `Event.AT_TARGET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub const AT_TARGET: u16 = 2u64 as u16;
    #[doc = "The `Event.BUBBLING_PHASE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Event`*"]
    pub const BUBBLING_PHASE: u16 = 3u64 as u16;
}
