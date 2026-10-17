#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaEncryptedEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaEncryptedEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `initDataType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initDataType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
impl MediaEncryptedEvent {
    #[js::import = "$0.initDataType"]
pub fn init_data_type(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `initData` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/initData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
impl MediaEncryptedEvent {
    #[js::import = "$0.initData"]
pub fn init_data(&self) -> Result<Option<::js_sys::ArrayBuffer>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`*"]
impl MediaEncryptedEvent {
    #[js::import = "new MediaEncryptedEvent($0)"]
pub fn new(type_: &str) -> Result<MediaEncryptedEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaKeyNeededEventInit")]
    #[doc = "The `new MediaEncryptedEvent(..)` constructor, creating a new instance of `MediaEncryptedEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaEncryptedEvent/MediaEncryptedEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaEncryptedEvent`, `MediaKeyNeededEventInit`*"]
impl MediaEncryptedEvent {
    #[js::import = "new MediaEncryptedEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &MediaKeyNeededEventInit,) -> Result<MediaEncryptedEvent, JsValue> { panic!("web-sys intrinsic") }
}
