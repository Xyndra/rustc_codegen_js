#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceProximityEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceProximityEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
impl DeviceProximityEvent {
    #[js::import = "$0.value"]
pub fn value(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `min` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/min)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
impl DeviceProximityEvent {
    #[js::import = "$0.min"]
pub fn min(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
impl DeviceProximityEvent {
    #[js::import = "$0.max"]
pub fn max(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`*"]
impl DeviceProximityEvent {
    #[js::import = "new DeviceProximityEvent($0)"]
pub fn new(type_: &str) -> Result<DeviceProximityEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DeviceProximityEventInit")]
    #[doc = "The `new DeviceProximityEvent(..)` constructor, creating a new instance of `DeviceProximityEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceProximityEvent/DeviceProximityEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEvent`, `DeviceProximityEventInit`*"]
impl DeviceProximityEvent {
    #[js::import = "new DeviceProximityEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &DeviceProximityEventInit,) -> Result<DeviceProximityEvent, JsValue> { panic!("web-sys intrinsic") }
}
