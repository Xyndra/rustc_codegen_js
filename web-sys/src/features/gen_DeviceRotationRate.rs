#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceRotationRate` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceRotationRate(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/alpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
impl DeviceRotationRate {
    #[js::import = "$0.alpha"]
pub fn alpha(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `beta` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/beta)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
impl DeviceRotationRate {
    #[js::import = "$0.beta"]
pub fn beta(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DeviceRotationRate/gamma)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRate`*"]
impl DeviceRotationRate {
    #[js::import = "$0.gamma"]
pub fn gamma(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
