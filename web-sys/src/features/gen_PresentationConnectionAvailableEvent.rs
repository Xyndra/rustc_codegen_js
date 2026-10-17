#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionAvailableEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PresentationConnectionAvailableEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PresentationConnection")]
    #[doc = "Getter for the `connection` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/connection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnection`, `PresentationConnectionAvailableEvent`*"]
impl PresentationConnectionAvailableEvent {
    #[js::import = "$0.connection"]
pub fn connection(&self) -> PresentationConnection { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PresentationConnectionAvailableEventInit")]
    #[doc = "The `new PresentationConnectionAvailableEvent(..)` constructor, creating a new instance of `PresentationConnectionAvailableEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionAvailableEvent/PresentationConnectionAvailableEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionAvailableEvent`, `PresentationConnectionAvailableEventInit`*"]
impl PresentationConnectionAvailableEvent {
    #[js::import = "new PresentationConnectionAvailableEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &PresentationConnectionAvailableEventInit,) -> Result<PresentationConnectionAvailableEvent, JsValue> { panic!("web-sys intrinsic") }
}
