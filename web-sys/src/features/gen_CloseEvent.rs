#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CloseEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CloseEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `wasClean` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/wasClean)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
impl CloseEvent {
    #[js::import = "$0.wasClean"]
pub fn was_clean(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
impl CloseEvent {
    #[js::import = "$0.code"]
pub fn code(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
impl CloseEvent {
    #[js::import = "$0.reason"]
pub fn reason(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`*"]
impl CloseEvent {
    #[js::import = "new CloseEvent($0)"]
pub fn new(type_: &str) -> Result<CloseEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CloseEventInit")]
    #[doc = "The `new CloseEvent(..)` constructor, creating a new instance of `CloseEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/CloseEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEvent`, `CloseEventInit`*"]
impl CloseEvent {
    #[js::import = "new CloseEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &CloseEventInit,) -> Result<CloseEvent, JsValue> { panic!("web-sys intrinsic") }
}
