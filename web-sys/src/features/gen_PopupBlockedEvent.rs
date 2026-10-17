#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopupBlockedEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PopupBlockedEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `requestingWindow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/requestingWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`, `Window`*"]
impl PopupBlockedEvent {
    #[js::import = "$0.requestingWindow"]
pub fn requesting_window(&self) -> Option<Window> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `popupWindowName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
impl PopupBlockedEvent {
    #[js::import = "$0.popupWindowName"]
pub fn popup_window_name(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `popupWindowFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/popupWindowFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
impl PopupBlockedEvent {
    #[js::import = "$0.popupWindowFeatures"]
pub fn popup_window_features(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`*"]
impl PopupBlockedEvent {
    #[js::import = "new PopupBlockedEvent($0)"]
pub fn new(type_: &str) -> Result<PopupBlockedEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PopupBlockedEventInit")]
    #[doc = "The `new PopupBlockedEvent(..)` constructor, creating a new instance of `PopupBlockedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PopupBlockedEvent/PopupBlockedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopupBlockedEvent`, `PopupBlockedEventInit`*"]
impl PopupBlockedEvent {
    #[js::import = "new PopupBlockedEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &PopupBlockedEventInit,) -> Result<PopupBlockedEvent, JsValue> { panic!("web-sys intrinsic") }
}
