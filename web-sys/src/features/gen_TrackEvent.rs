#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TrackEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TrackEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEvent`*"]
impl TrackEvent {
    #[js::import = "$0.track"]
pub fn track(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new TrackEvent(..)` constructor, creating a new instance of `TrackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/TrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEvent`*"]
impl TrackEvent {
    #[js::import = "new TrackEvent($0)"]
pub fn new(type_: &str) -> Result<TrackEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TrackEventInit")]
    #[doc = "The `new TrackEvent(..)` constructor, creating a new instance of `TrackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TrackEvent/TrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TrackEvent`, `TrackEventInit`*"]
impl TrackEvent {
    #[js::import = "new TrackEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &TrackEventInit,) -> Result<TrackEvent, JsValue> { panic!("web-sys intrinsic") }
}
