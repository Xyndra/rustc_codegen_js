#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MessageEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MessageEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.origin"]
pub fn origin(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lastEventId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/lastEventId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.lastEventId"]
pub fn last_event_id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.source"]
pub fn source(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ports` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/ports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.ports"]
pub fn ports(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `new MessageEvent(..)` constructor, creating a new instance of `MessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/MessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "new MessageEvent($0)"]
pub fn new(type_: &str) -> Result<MessageEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MessageEventInit")]
    #[doc = "The `new MessageEvent(..)` constructor, creating a new instance of `MessageEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/MessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `MessageEventInit`*"]
impl MessageEvent {
    #[js::import = "new MessageEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &MessageEventInit,) -> Result<MessageEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1)"]
pub fn init_message_event(&self, type_: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2)"]
pub fn init_message_event_with_bubbles(&self, type_: &str, bubbles: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3)"]
pub fn init_message_event_with_bubbles_and_cancelable(&self, type_: &str, bubbles: bool, cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3, $4)"]
pub fn init_message_event_with_bubbles_and_cancelable_and_data(&self, type_: &str, bubbles: bool, cancelable: bool, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3, $4, $5)"]
pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin(&self, type_: &str, bubbles: bool, cancelable: bool, data: &::wasm_bindgen::JsValue, origin: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id(&self, type_: &str, bubbles: bool, cancelable: bool, data: &::wasm_bindgen::JsValue, origin: &str, last_event_id: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `Window`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window(&self, type_: &str, bubbles: bool, cancelable: bool, data: &::wasm_bindgen::JsValue, origin: &str, last_event_id: &str, source: Option<&Window>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MessagePort")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `MessagePort`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port(&self, type_: &str, bubbles: bool, cancelable: bool, data: &::wasm_bindgen::JsValue, origin: &str, last_event_id: &str, source: Option<&MessagePort>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServiceWorker")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `ServiceWorker`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker(&self, type_: &str, bubbles: bool, cancelable: bool, data: &::wasm_bindgen::JsValue, origin: &str, last_event_id: &str, source: Option<&ServiceWorker>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `Window`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_window_and_ports(&self, type_: &str, bubbles: bool, cancelable: bool, data: &::wasm_bindgen::JsValue, origin: &str, last_event_id: &str, source: Option<&Window>, ports: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MessagePort")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `MessagePort`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_message_port_and_ports(&self, type_: &str, bubbles: bool, cancelable: bool, data: &::wasm_bindgen::JsValue, origin: &str, last_event_id: &str, source: Option<&MessagePort>, ports: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServiceWorker")]
    #[doc = "The `initMessageEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent/initMessageEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MessageEvent`, `ServiceWorker`*"]
impl MessageEvent {
    #[js::import = "$0.initMessageEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_message_event_with_bubbles_and_cancelable_and_data_and_origin_and_last_event_id_and_opt_service_worker_and_ports(&self, type_: &str, bubbles: bool, cancelable: bool, data: &::wasm_bindgen::JsValue, origin: &str, last_event_id: &str, source: Option<&ServiceWorker>, ports: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
