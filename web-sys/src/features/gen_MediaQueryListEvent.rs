#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaQueryListEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaQueryListEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
impl MediaQueryListEvent {
    #[js::import = "$0.media"]
pub fn media(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `matches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/matches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
impl MediaQueryListEvent {
    #[js::import = "$0.matches"]
pub fn matches(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`*"]
impl MediaQueryListEvent {
    #[js::import = "new MediaQueryListEvent($0)"]
pub fn new(type_: &str) -> Result<MediaQueryListEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaQueryListEventInit")]
    #[doc = "The `new MediaQueryListEvent(..)` constructor, creating a new instance of `MediaQueryListEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryListEvent/MediaQueryListEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryListEvent`, `MediaQueryListEventInit`*"]
impl MediaQueryListEvent {
    #[js::import = "new MediaQueryListEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &MediaQueryListEventInit,) -> Result<MediaQueryListEvent, JsValue> { panic!("web-sys intrinsic") }
}
