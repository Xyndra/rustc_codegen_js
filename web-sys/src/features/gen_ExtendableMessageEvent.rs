#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtendableMessageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ExtendableMessageEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
impl ExtendableMessageEvent {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
impl ExtendableMessageEvent {
    #[js::import = "$0.origin"]
pub fn origin(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/lastEventId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
impl ExtendableMessageEvent {
    #[js::import = "$0.lastEventId"]
pub fn last_event_id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
impl ExtendableMessageEvent {
    #[js::import = "$0.source"]
pub fn source(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ports` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
impl ExtendableMessageEvent {
    #[js::import = "$0.ports"]
pub fn ports(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`*"]
impl ExtendableMessageEvent {
    #[js::import = "new ExtendableMessageEvent($0)"]
pub fn new(type_: &str) -> Result<ExtendableMessageEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ExtendableMessageEventInit")]
    #[doc = "The `new ExtendableMessageEvent(..)` constructor, creating a new instance of `ExtendableMessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ExtendableMessageEvent/ExtendableMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtendableMessageEvent`, `ExtendableMessageEventInit`*"]
impl ExtendableMessageEvent {
    #[js::import = "new ExtendableMessageEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &ExtendableMessageEventInit,) -> Result<ExtendableMessageEvent, JsValue> { panic!("web-sys intrinsic") }
}
