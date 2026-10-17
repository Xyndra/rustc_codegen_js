#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrFrameData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrFrameData(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
impl VrFrameData {
    #[js::import = "$0.timestamp"]
pub fn timestamp(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `leftProjectionMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftProjectionMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
impl VrFrameData {
    #[js::import = "$0.leftProjectionMatrix"]
pub fn left_projection_matrix(&self) -> Result<::alloc::vec::Vec<f32>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `leftViewMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/leftViewMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
impl VrFrameData {
    #[js::import = "$0.leftViewMatrix"]
pub fn left_view_matrix(&self) -> Result<::alloc::vec::Vec<f32>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rightProjectionMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightProjectionMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
impl VrFrameData {
    #[js::import = "$0.rightProjectionMatrix"]
pub fn right_projection_matrix(&self) -> Result<::alloc::vec::Vec<f32>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rightViewMatrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/rightViewMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
impl VrFrameData {
    #[js::import = "$0.rightViewMatrix"]
pub fn right_view_matrix(&self) -> Result<::alloc::vec::Vec<f32>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VrPose")]
    #[doc = "Getter for the `pose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/pose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`, `VrPose`*"]
impl VrFrameData {
    #[js::import = "$0.pose"]
pub fn pose(&self) -> VrPose { panic!("web-sys intrinsic") }
}
    #[doc = "The `new VrFrameData(..)` constructor, creating a new instance of `VrFrameData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFrameData/VRFrameData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFrameData`*"]
impl VrFrameData {
    #[js::import = "new VRFrameData()"]
pub fn new() -> Result<VrFrameData, JsValue> { panic!("web-sys intrinsic") }
}
