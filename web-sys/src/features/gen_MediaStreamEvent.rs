#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStreamEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStream")]
    #[doc = "Getter for the `stream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent/stream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamEvent`*"]
impl MediaStreamEvent {
    #[js::import = "$0.stream"]
pub fn stream(&self) -> Option<MediaStream> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MediaStreamEvent(..)` constructor, creating a new instance of `MediaStreamEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent/MediaStreamEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEvent`*"]
impl MediaStreamEvent {
    #[js::import = "new MediaStreamEvent($0)"]
pub fn new(type_: &str) -> Result<MediaStreamEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamEventInit")]
    #[doc = "The `new MediaStreamEvent(..)` constructor, creating a new instance of `MediaStreamEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamEvent/MediaStreamEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamEvent`, `MediaStreamEventInit`*"]
impl MediaStreamEvent {
    #[js::import = "new MediaStreamEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &MediaStreamEventInit,) -> Result<MediaStreamEvent, JsValue> { panic!("web-sys intrinsic") }
}
