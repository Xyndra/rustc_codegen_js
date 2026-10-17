#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuComputePassEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuComputePassEncoder(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.label = $1"]
pub fn set_label(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `dispatchWorkgroups()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchWorkgroups)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.dispatchWorkgroups($1)"]
pub fn dispatch_workgroups(&self, workgroup_count_x: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `dispatchWorkgroups()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchWorkgroups)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.dispatchWorkgroups($1, $2)"]
pub fn dispatch_workgroups_with_workgroup_count_y(&self, workgroup_count_x: u32, workgroup_count_y: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `dispatchWorkgroups()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchWorkgroups)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.dispatchWorkgroups($1, $2, $3)"]
pub fn dispatch_workgroups_with_workgroup_count_y_and_workgroup_count_z(&self, workgroup_count_x: u32, workgroup_count_y: u32, workgroup_count_z: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `dispatchWorkgroupsIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchWorkgroupsIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.dispatchWorkgroupsIndirect($1, $2)"]
pub fn dispatch_workgroups_indirect_with_u32(&self, indirect_buffer: &GpuBuffer, indirect_offset: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `dispatchWorkgroupsIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchWorkgroupsIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.dispatchWorkgroupsIndirect($1, $2)"]
pub fn dispatch_workgroups_indirect_with_f64(&self, indirect_buffer: &GpuBuffer, indirect_offset: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `end()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/end)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.end()"]
pub fn end(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuComputePipeline")]
    #[doc = "The `setPipeline()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setPipeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`, `GpuComputePipeline`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.setPipeline($1)"]
pub fn set_pipeline(&self, pipeline: &GpuComputePipeline) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.setBindGroup($1, $2)"]
pub fn set_bind_group(&self, index: u32, bind_group: Option<&GpuBindGroup>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3)"]
pub fn set_bind_group_with_u32_sequence(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3, $4, $5)"]
pub fn set_bind_group_with_u32_slice_and_u32_and_dynamic_offsets_data_length(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets_data: &[u32], dynamic_offsets_data_start: u32, dynamic_offsets_data_length: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3, $4, $5)"]
pub fn set_bind_group_with_u32_array_and_u32_and_dynamic_offsets_data_length(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets_data: &::js_sys::Uint32Array, dynamic_offsets_data_start: u32, dynamic_offsets_data_length: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3, $4, $5)"]
pub fn set_bind_group_with_u32_slice_and_f64_and_dynamic_offsets_data_length(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets_data: &[u32], dynamic_offsets_data_start: f64, dynamic_offsets_data_length: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3, $4, $5)"]
pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets_data: &::js_sys::Uint32Array, dynamic_offsets_data_start: f64, dynamic_offsets_data_length: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `insertDebugMarker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/insertDebugMarker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.insertDebugMarker($1)"]
pub fn insert_debug_marker(&self, marker_label: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `popDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/popDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.popDebugGroup()"]
pub fn pop_debug_group(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `pushDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/pushDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuComputePassEncoder {
    #[js::import = "$0.pushDebugGroup($1)"]
pub fn push_debug_group(&self, group_label: &str) { panic!("web-sys intrinsic") }
}
