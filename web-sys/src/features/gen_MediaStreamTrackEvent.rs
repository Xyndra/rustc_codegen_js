#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamTrackEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStreamTrackEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStreamTrack")]
    #[doc = "Getter for the `track` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent/track)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrack`, `MediaStreamTrackEvent`*"]
impl MediaStreamTrackEvent {
    #[js::import = "$0.track"]
pub fn track(&self) -> MediaStreamTrack { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaStreamTrackEventInit")]
    #[doc = "The `new MediaStreamTrackEvent(..)` constructor, creating a new instance of `MediaStreamTrackEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamTrackEvent/MediaStreamTrackEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamTrackEvent`, `MediaStreamTrackEventInit`*"]
impl MediaStreamTrackEvent {
    #[js::import = "new MediaStreamTrackEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &MediaStreamTrackEventInit,) -> Result<MediaStreamTrackEvent, JsValue> { panic!("web-sys intrinsic") }
}
