#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceOrientationEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceOrientationEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/alpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.alpha"]
pub fn alpha(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `beta` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/beta)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.beta"]
pub fn beta(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/gamma)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.gamma"]
pub fn gamma(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `absolute` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/absolute)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.absolute"]
pub fn absolute(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DeviceOrientationEvent(..)` constructor, creating a new instance of `DeviceOrientationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/DeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "new DeviceOrientationEvent($0)"]
pub fn new(type_: &str) -> Result<DeviceOrientationEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DeviceOrientationEventInit")]
    #[doc = "The `new DeviceOrientationEvent(..)` constructor, creating a new instance of `DeviceOrientationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/DeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`, `DeviceOrientationEventInit`*"]
impl DeviceOrientationEvent {
    #[js::import = "new DeviceOrientationEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &DeviceOrientationEventInit,) -> Result<DeviceOrientationEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDeviceOrientationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.initDeviceOrientationEvent($1)"]
pub fn init_device_orientation_event(&self, type_: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDeviceOrientationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.initDeviceOrientationEvent($1, $2)"]
pub fn init_device_orientation_event_with_can_bubble(&self, type_: &str, can_bubble: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDeviceOrientationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.initDeviceOrientationEvent($1, $2, $3)"]
pub fn init_device_orientation_event_with_can_bubble_and_cancelable(&self, type_: &str, can_bubble: bool, cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDeviceOrientationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.initDeviceOrientationEvent($1, $2, $3, $4)"]
pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha(&self, type_: &str, can_bubble: bool, cancelable: bool, alpha: Option<f64>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDeviceOrientationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.initDeviceOrientationEvent($1, $2, $3, $4, $5)"]
pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta(&self, type_: &str, can_bubble: bool, cancelable: bool, alpha: Option<f64>, beta: Option<f64>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDeviceOrientationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.initDeviceOrientationEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma(&self, type_: &str, can_bubble: bool, cancelable: bool, alpha: Option<f64>, beta: Option<f64>, gamma: Option<f64>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initDeviceOrientationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceOrientationEvent/initDeviceOrientationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEvent`*"]
impl DeviceOrientationEvent {
    #[js::import = "$0.initDeviceOrientationEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_device_orientation_event_with_can_bubble_and_cancelable_and_alpha_and_beta_and_gamma_and_absolute(&self, type_: &str, can_bubble: bool, cancelable: bool, alpha: Option<f64>, beta: Option<f64>, gamma: Option<f64>, absolute: bool) { panic!("web-sys intrinsic") }
}
