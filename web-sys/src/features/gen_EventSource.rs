#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventSource` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
#[js::opaque]
#[derive(Copy)]
pub struct EventSource(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.url"]
pub fn url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/withCredentials)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.withCredentials"]
pub fn with_credentials(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.onopen"]
pub fn onopen(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onopen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onopen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.onopen = $1"]
pub fn set_onopen(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new EventSource(..)` constructor, creating a new instance of `EventSource`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "new EventSource($0)"]
pub fn new(url: &str) -> Result<EventSource, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventSourceInit")]
    #[doc = "The `new EventSource(..)` constructor, creating a new instance of `EventSource`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/EventSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`, `EventSourceInit`*"]
impl EventSource {
    #[js::import = "new EventSource($0, $1)"]
pub fn new_with_event_source_init_dict(url: &str, event_source_init_dict: &EventSourceInit,) -> Result<EventSource, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EventSource/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
impl EventSource {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
impl EventSource {
    #[doc = "The `EventSource.CONNECTING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub const CONNECTING: u16 = 0i64 as u16;
    #[doc = "The `EventSource.OPEN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub const OPEN: u16 = 1u64 as u16;
    #[doc = "The `EventSource.CLOSED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSource`*"]
    pub const CLOSED: u16 = 2u64 as u16;
}
