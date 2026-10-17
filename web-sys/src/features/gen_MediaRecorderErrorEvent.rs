#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorderErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaRecorderErrorEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomException")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`, `MediaRecorderErrorEvent`*"]
impl MediaRecorderErrorEvent {
    #[js::import = "$0.error"]
pub fn error(&self) -> DomException { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaRecorderErrorEventInit")]
    #[doc = "The `new MediaRecorderErrorEvent(..)` constructor, creating a new instance of `MediaRecorderErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaRecorderErrorEvent/MediaRecorderErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderErrorEvent`, `MediaRecorderErrorEventInit`*"]
impl MediaRecorderErrorEvent {
    #[js::import = "new MediaRecorderErrorEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &MediaRecorderErrorEventInit,) -> Result<MediaRecorderErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}
