#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrFrame` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct XrFrame(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSession")]
    #[doc = "Getter for the `session` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/session)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrSession`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrFrame {
    #[js::import = "$0.session"]
pub fn session(&self) -> XrSession { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `predictedDisplayTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/predictedDisplayTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrFrame {
    #[js::import = "$0.predictedDisplayTime"]
pub fn predicted_display_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `fillJointRadii()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillJointRadii)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrFrame {
    #[js::import = "$0.fillJointRadii($1, $2)"]
pub fn fill_joint_radii_with_f32_slice(&self, joint_spaces: &::wasm_bindgen::JsValue, radii: &mut [f32]) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `fillJointRadii()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillJointRadii)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrFrame {
    #[js::import = "$0.fillJointRadii($1, $2)"]
pub fn fill_joint_radii_with_f32_array(&self, joint_spaces: &::wasm_bindgen::JsValue, radii: &::js_sys::Float32Array) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[doc = "The `fillPoses()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillPoses)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrFrame {
    #[js::import = "$0.fillPoses($1, $2, $3)"]
pub fn fill_poses_with_f32_slice(&self, spaces: &::wasm_bindgen::JsValue, base_space: &XrSpace, transforms: &mut [f32]) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSpace")]
    #[doc = "The `fillPoses()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/fillPoses)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrFrame {
    #[js::import = "$0.fillPoses($1, $2, $3)"]
pub fn fill_poses_with_f32_array(&self, spaces: &::wasm_bindgen::JsValue, base_space: &XrSpace, transforms: &::js_sys::Float32Array) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrJointPose", feature = "XrJointSpace", feature = "XrSpace",))]
    #[doc = "The `getJointPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getJointPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrJointPose`, `XrJointSpace`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrFrame {
    #[js::import = "$0.getJointPose($1, $2)"]
pub fn get_joint_pose(&self, joint: &XrJointSpace, base_space: &XrSpace) -> Option<XrJointPose> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrPose", feature = "XrSpace",))]
    #[doc = "The `getPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrPose`, `XrSpace`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrFrame {
    #[js::import = "$0.getPose($1, $2)"]
pub fn get_pose(&self, space: &XrSpace, base_space: &XrSpace) -> Option<XrPose> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrReferenceSpace", feature = "XrViewerPose",))]
    #[doc = "The `getViewerPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRFrame/getViewerPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrFrame`, `XrReferenceSpace`, `XrViewerPose`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrFrame {
    #[js::import = "$0.getViewerPose($1)"]
pub fn get_viewer_pose(&self, reference_space: &XrReferenceSpace) -> Option<XrViewerPose> { panic!("web-sys intrinsic") }
}
