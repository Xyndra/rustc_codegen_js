#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TimeEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TimeEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `detail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/detail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`*"]
impl TimeEvent {
    #[js::import = "$0.detail"]
pub fn detail(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `view` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/view)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
impl TimeEvent {
    #[js::import = "$0.view"]
pub fn view(&self) -> Option<Window> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initTimeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`*"]
impl TimeEvent {
    #[js::import = "$0.initTimeEvent($1)"]
pub fn init_time_event(&self, a_type: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initTimeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
impl TimeEvent {
    #[js::import = "$0.initTimeEvent($1, $2)"]
pub fn init_time_event_with_a_view(&self, a_type: &str, a_view: Option<&Window>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initTimeEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeEvent/initTimeEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeEvent`, `Window`*"]
impl TimeEvent {
    #[js::import = "$0.initTimeEvent($1, $2, $3)"]
pub fn init_time_event_with_a_view_and_a_detail(&self, a_type: &str, a_view: Option<&Window>, a_detail: i32) { panic!("web-sys intrinsic") }
}
