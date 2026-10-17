#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceAcceleration` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceAcceleration(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
impl DeviceAcceleration {
    #[js::import = "$0.x"]
pub fn x(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
impl DeviceAcceleration {
    #[js::import = "$0.y"]
pub fn y(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceAcceleration/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAcceleration`*"]
impl DeviceAcceleration {
    #[js::import = "$0.z"]
pub fn z(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
