#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrPose` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrPose(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
impl VrPose {
    #[js::import = "$0.position"]
pub fn position(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `linearVelocity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/linearVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
impl VrPose {
    #[js::import = "$0.linearVelocity"]
pub fn linear_velocity(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `linearAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/linearAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
impl VrPose {
    #[js::import = "$0.linearAcceleration"]
pub fn linear_acceleration(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/orientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
impl VrPose {
    #[js::import = "$0.orientation"]
pub fn orientation(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `angularVelocity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/angularVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
impl VrPose {
    #[js::import = "$0.angularVelocity"]
pub fn angular_velocity(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `angularAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRPose/angularAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrPose`*"]
impl VrPose {
    #[js::import = "$0.angularAcceleration"]
pub fn angular_acceleration(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
