#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionCloseEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PresentationConnectionCloseEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PresentationConnectionClosedReason")]
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionClosedReason`*"]
impl PresentationConnectionCloseEvent {
    #[js::import = "$0.reason"]
pub fn reason(&self) -> PresentationConnectionClosedReason { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`*"]
impl PresentationConnectionCloseEvent {
    #[js::import = "$0.message"]
pub fn message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PresentationConnectionCloseEventInit")]
    #[doc = "The `new PresentationConnectionCloseEvent(..)` constructor, creating a new instance of `PresentationConnectionCloseEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionCloseEvent/PresentationConnectionCloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionCloseEvent`, `PresentationConnectionCloseEventInit`*"]
impl PresentationConnectionCloseEvent {
    #[js::import = "new PresentationConnectionCloseEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &PresentationConnectionCloseEventInit,) -> Result<PresentationConnectionCloseEvent, JsValue> { panic!("web-sys intrinsic") }
}
