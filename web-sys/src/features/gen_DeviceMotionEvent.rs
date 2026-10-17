#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceMotionEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceMotionEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DeviceAcceleration")]
    #[doc = "Getter for the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/acceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*"]
impl DeviceMotionEvent {
    #[js::import = "$0.acceleration"]
pub fn acceleration(&self) -> Option<DeviceAcceleration> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DeviceAcceleration")]
    #[doc = "Getter for the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/accelerationIncludingGravity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`, `DeviceMotionEvent`*"]
impl DeviceMotionEvent {
    #[js::import = "$0.accelerationIncludingGravity"]
pub fn acceleration_including_gravity(&self) -> Option<DeviceAcceleration> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DeviceRotationRate")]
    #[doc = "Getter for the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/rotationRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceRotationRate`*"]
impl DeviceMotionEvent {
    #[js::import = "$0.rotationRate"]
pub fn rotation_rate(&self) -> Option<DeviceRotationRate> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `interval` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/interval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
impl DeviceMotionEvent {
    #[js::import = "$0.interval"]
pub fn interval(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`*"]
impl DeviceMotionEvent {
    #[js::import = "new DeviceMotionEvent($0)"]
pub fn new(type_: &str) -> Result<DeviceMotionEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DeviceMotionEventInit")]
    #[doc = "The `new DeviceMotionEvent(..)` constructor, creating a new instance of `DeviceMotionEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/DeviceMotionEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEvent`, `DeviceMotionEventInit`*"]
impl DeviceMotionEvent {
    #[js::import = "new DeviceMotionEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &DeviceMotionEventInit,) -> Result<DeviceMotionEvent, JsValue> { panic!("web-sys intrinsic") }
}
