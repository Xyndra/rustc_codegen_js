#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrMockDisplay` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrMockDisplay(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "VrEye")]
    #[doc = "The `setEyeParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setEyeParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrEye`, `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setEyeParameter($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn set_eye_parameter(&self, eye: VrEye, offset_x: f64, offset_y: f64, offset_z: f64, up_degree: f64, right_degree: f64, down_degree: f64, left_degree: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setEyeResolution()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setEyeResolution)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setEyeResolution($1, $2)"]
pub fn set_eye_resolution(&self, a_render_width: u32, a_render_height: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setMountState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setMountState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setMountState($1)"]
pub fn set_mount_state(&self, is_mounted: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&mut [f32]>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&mut [f32]>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&mut [f32]>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&mut [f32]>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_slice_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&mut [f32]>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPose()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/setPose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.setPose($1, $2, $3, $4, $5, $6)"]
pub fn set_pose_with_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array_and_opt_f32_array(&self, position: Option<&::js_sys::Float32Array>, linear_velocity: Option<&::js_sys::Float32Array>, linear_acceleration: Option<&::js_sys::Float32Array>, orientation: Option<&::js_sys::Float32Array>, angular_velocity: Option<&::js_sys::Float32Array>, angular_acceleration: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRMockDisplay/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrMockDisplay`*"]
impl VrMockDisplay {
    #[js::import = "$0.update()"]
pub fn update(&self) { panic!("web-sys intrinsic") }
}
