#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrDisplay` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrDisplay(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `isConnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isConnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.isConnected"]
pub fn is_connected(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isPresenting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/isPresenting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.isPresenting"]
pub fn is_presenting(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VrDisplayCapabilities")]
    #[doc = "Getter for the `capabilities` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/capabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrDisplayCapabilities`*"]
impl VrDisplay {
    #[js::import = "$0.capabilities"]
pub fn capabilities(&self) -> VrDisplayCapabilities { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VrStageParameters")]
    #[doc = "Getter for the `stageParameters` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/stageParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrStageParameters`*"]
impl VrDisplay {
    #[js::import = "$0.stageParameters"]
pub fn stage_parameters(&self) -> Option<VrStageParameters> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `displayId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.displayId"]
pub fn display_id(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/displayName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.displayName"]
pub fn display_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.depthNear"]
pub fn depth_near(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `depthNear` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthNear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.depthNear = $1"]
pub fn set_depth_near(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.depthFar"]
pub fn depth_far(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `depthFar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/depthFar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.depthFar = $1"]
pub fn set_depth_far(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancelAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/cancelAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.cancelAnimationFrame($1)"]
pub fn cancel_animation_frame(&self, handle: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `exitPresent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/exitPresent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.exitPresent()"]
pub fn exit_present(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "VrEye", feature = "VrEyeParameters",))]
    #[doc = "The `getEyeParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getEyeParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrEye`, `VrEyeParameters`*"]
impl VrDisplay {
    #[js::import = "$0.getEyeParameters($1)"]
pub fn get_eye_parameters(&self, which_eye: VrEye) -> VrEyeParameters { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VrFrameData")]
    #[doc = "The `getFrameData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getFrameData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrFrameData`*"]
impl VrDisplay {
    #[js::import = "$0.getFrameData($1)"]
pub fn get_frame_data(&self, frame_data: &VrFrameData) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `getLayers()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getLayers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.getLayers()"]
pub fn get_layers(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VrPose")]
    #[doc = "The `getPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrPose`*"]
impl VrDisplay {
    #[js::import = "$0.getPose()"]
pub fn get_pose(&self) -> VrPose { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VrSubmitFrameResult")]
    #[doc = "The `getSubmitFrameResult()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/getSubmitFrameResult)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`, `VrSubmitFrameResult`*"]
impl VrDisplay {
    #[js::import = "$0.getSubmitFrameResult($1)"]
pub fn get_submit_frame_result(&self, result: &VrSubmitFrameResult) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.requestAnimationFrame($1)"]
pub fn request_animation_frame(&self, callback: &::js_sys::Function) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestPresent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/requestPresent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.requestPresent($1)"]
pub fn request_present(&self, layers: &::wasm_bindgen::JsValue) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `resetPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/resetPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.resetPose()"]
pub fn reset_pose(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `submitFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRDisplay/submitFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrDisplay`*"]
impl VrDisplay {
    #[js::import = "$0.submitFrame()"]
pub fn submit_frame(&self) { panic!("web-sys intrinsic") }
}
