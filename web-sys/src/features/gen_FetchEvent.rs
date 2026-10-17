#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FetchEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Request")]
    #[doc = "Getter for the `request` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`, `Request`*"]
impl FetchEvent {
    #[js::import = "$0.request"]
pub fn request(&self) -> Request { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/clientId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`*"]
impl FetchEvent {
    #[js::import = "$0.clientId"]
pub fn client_id(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isReload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/isReload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`*"]
impl FetchEvent {
    #[js::import = "$0.isReload"]
pub fn is_reload(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FetchEventInit")]
    #[doc = "The `new FetchEvent(..)` constructor, creating a new instance of `FetchEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/FetchEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`, `FetchEventInit`*"]
impl FetchEvent {
    #[js::import = "new FetchEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &FetchEventInit) -> Result<FetchEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `respondWith()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FetchEvent/respondWith)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEvent`*"]
impl FetchEvent {
    #[js::import = "$0.respondWith($1)"]
pub fn respond_with(&self, r: &::js_sys::Promise) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
