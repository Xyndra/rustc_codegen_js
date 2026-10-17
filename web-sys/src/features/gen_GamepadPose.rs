#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadPose` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
#[js::opaque]
#[derive(Copy)]
pub struct GamepadPose(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `hasOrientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
impl GamepadPose {
    #[js::import = "$0.hasOrientation"]
pub fn has_orientation(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hasPosition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
impl GamepadPose {
    #[js::import = "$0.hasPosition"]
pub fn has_position(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
impl GamepadPose {
    #[js::import = "$0.position"]
pub fn position(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `linearVelocity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
impl GamepadPose {
    #[js::import = "$0.linearVelocity"]
pub fn linear_velocity(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `linearAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
impl GamepadPose {
    #[js::import = "$0.linearAcceleration"]
pub fn linear_acceleration(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/orientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
impl GamepadPose {
    #[js::import = "$0.orientation"]
pub fn orientation(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `angularVelocity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
impl GamepadPose {
    #[js::import = "$0.angularVelocity"]
pub fn angular_velocity(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `angularAcceleration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularAcceleration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadPose`*"]
impl GamepadPose {
    #[js::import = "$0.angularAcceleration"]
pub fn angular_acceleration(&self) -> Result<Option<::alloc::vec::Vec<f32>>, JsValue> { panic!("web-sys intrinsic") }
}
