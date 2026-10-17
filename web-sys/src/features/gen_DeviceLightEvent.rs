#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceLightEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceLightEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
impl DeviceLightEvent {
    #[js::import = "$0.value"]
pub fn value(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DeviceLightEvent(..)` constructor, creating a new instance of `DeviceLightEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/DeviceLightEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEvent`*"]
impl DeviceLightEvent {
    #[js::import = "new DeviceLightEvent($0)"]
pub fn new(type_: &str) -> Result<DeviceLightEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DeviceLightEventInit")]
    #[doc = "The `new DeviceLightEvent(..)` constructor, creating a new instance of `DeviceLightEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceLightEvent/DeviceLightEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceLightEvent`, `DeviceLightEventInit`*"]
impl DeviceLightEvent {
    #[js::import = "new DeviceLightEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &DeviceLightEventInit,) -> Result<DeviceLightEvent, JsValue> { panic!("web-sys intrinsic") }
}
