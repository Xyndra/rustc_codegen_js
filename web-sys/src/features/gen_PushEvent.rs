#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PushMessageData")]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEvent`, `PushMessageData`*"]
impl PushEvent {
    #[js::import = "$0.data"]
pub fn data(&self) -> Option<PushMessageData> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new PushEvent(..)` constructor, creating a new instance of `PushEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/PushEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEvent`*"]
impl PushEvent {
    #[js::import = "new PushEvent($0)"]
pub fn new(type_: &str) -> Result<PushEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PushEventInit")]
    #[doc = "The `new PushEvent(..)` constructor, creating a new instance of `PushEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushEvent/PushEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEvent`, `PushEventInit`*"]
impl PushEvent {
    #[js::import = "new PushEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &PushEventInit,) -> Result<PushEvent, JsValue> { panic!("web-sys intrinsic") }
}
