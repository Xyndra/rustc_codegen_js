#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FocusEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FocusEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "EventTarget")]
    #[doc = "Getter for the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/relatedTarget)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `FocusEvent`*"]
impl FocusEvent {
    #[js::import = "$0.relatedTarget"]
pub fn related_target(&self) -> Option<EventTarget> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new FocusEvent(..)` constructor, creating a new instance of `FocusEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEvent`*"]
impl FocusEvent {
    #[js::import = "new FocusEvent($0)"]
pub fn new(type_arg: &str) -> Result<FocusEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FocusEventInit")]
    #[doc = "The `new FocusEvent(..)` constructor, creating a new instance of `FocusEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/FocusEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEvent`, `FocusEventInit`*"]
impl FocusEvent {
    #[js::import = "new FocusEvent($0, $1)"]
pub fn new_with_focus_event_init_dict(type_arg: &str, focus_event_init_dict: &FocusEventInit,) -> Result<FocusEvent, JsValue> { panic!("web-sys intrinsic") }
}
