#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyMessageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeyMessageEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaKeyMessageType")]
    #[doc = "Getter for the `messageType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/messageType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageType`*"]
impl MediaKeyMessageEvent {
    #[js::import = "$0.messageType"]
pub fn message_type(&self) -> MediaKeyMessageType { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEvent`*"]
impl MediaKeyMessageEvent {
    #[js::import = "$0.message"]
pub fn message(&self) -> Result<::js_sys::ArrayBuffer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaKeyMessageEventInit")]
    #[doc = "The `new MediaKeyMessageEvent(..)` constructor, creating a new instance of `MediaKeyMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyMessageEvent/MediaKeyMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyMessageEvent`, `MediaKeyMessageEventInit`*"]
impl MediaKeyMessageEvent {
    #[js::import = "new MediaKeyMessageEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &MediaKeyMessageEventInit,) -> Result<MediaKeyMessageEvent, JsValue> { panic!("web-sys intrinsic") }
}
