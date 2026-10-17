#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuRenderPassEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuRenderPassEncoder(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.label = $1"]
pub fn set_label(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `beginOcclusionQuery()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/beginOcclusionQuery)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.beginOcclusionQuery($1)"]
pub fn begin_occlusion_query(&self, query_index: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `end()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/end)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.end()"]
pub fn end(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `endOcclusionQuery()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/endOcclusionQuery)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.endOcclusionQuery()"]
pub fn end_occlusion_query(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `executeBundles()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/executeBundles)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.executeBundles($1)"]
pub fn execute_bundles(&self, bundles: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `setBlendConstant()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBlendConstant)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setBlendConstant($1)"]
pub fn set_blend_constant_with_f64_sequence(&self, color: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuColorDict")]
    #[doc = "The `setBlendConstant()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBlendConstant)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setBlendConstant($1)"]
pub fn set_blend_constant_with_gpu_color_dict(&self, color: &GpuColorDict) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `setScissorRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setScissorRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setScissorRect($1, $2, $3, $4)"]
pub fn set_scissor_rect(&self, x: u32, y: u32, width: u32, height: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `setStencilReference()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setStencilReference)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setStencilReference($1)"]
pub fn set_stencil_reference(&self, reference: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `setViewport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setViewport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setViewport($1, $2, $3, $4, $5, $6)"]
pub fn set_viewport(&self, x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setBindGroup($1, $2)"]
pub fn set_bind_group(&self, index: u32, bind_group: Option<&GpuBindGroup>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3)"]
pub fn set_bind_group_with_u32_sequence(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3, $4, $5)"]
pub fn set_bind_group_with_u32_slice_and_u32_and_dynamic_offsets_data_length(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets_data: &[u32], dynamic_offsets_data_start: u32, dynamic_offsets_data_length: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3, $4, $5)"]
pub fn set_bind_group_with_u32_array_and_u32_and_dynamic_offsets_data_length(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets_data: &::js_sys::Uint32Array, dynamic_offsets_data_start: u32, dynamic_offsets_data_length: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3, $4, $5)"]
pub fn set_bind_group_with_u32_slice_and_f64_and_dynamic_offsets_data_length(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets_data: &[u32], dynamic_offsets_data_start: f64, dynamic_offsets_data_length: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBindGroup")]
    #[doc = "The `setBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setBindGroup($1, $2, $3, $4, $5)"]
pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(&self, index: u32, bind_group: Option<&GpuBindGroup>, dynamic_offsets_data: &::js_sys::Uint32Array, dynamic_offsets_data_start: f64, dynamic_offsets_data_length: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `insertDebugMarker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/insertDebugMarker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.insertDebugMarker($1)"]
pub fn insert_debug_marker(&self, marker_label: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `popDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/popDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.popDebugGroup()"]
pub fn pop_debug_group(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `pushDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/pushDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.pushDebugGroup($1)"]
pub fn push_debug_group(&self, group_label: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `draw()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.draw($1)"]
pub fn draw(&self, vertex_count: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `draw()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.draw($1, $2)"]
pub fn draw_with_instance_count(&self, vertex_count: u32, instance_count: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `draw()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.draw($1, $2, $3)"]
pub fn draw_with_instance_count_and_first_vertex(&self, vertex_count: u32, instance_count: u32, first_vertex: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `draw()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.draw($1, $2, $3, $4)"]
pub fn draw_with_instance_count_and_first_vertex_and_first_instance(&self, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `drawIndexed()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.drawIndexed($1)"]
pub fn draw_indexed(&self, index_count: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `drawIndexed()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.drawIndexed($1, $2)"]
pub fn draw_indexed_with_instance_count(&self, index_count: u32, instance_count: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `drawIndexed()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.drawIndexed($1, $2, $3)"]
pub fn draw_indexed_with_instance_count_and_first_index(&self, index_count: u32, instance_count: u32, first_index: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `drawIndexed()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.drawIndexed($1, $2, $3, $4)"]
pub fn draw_indexed_with_instance_count_and_first_index_and_base_vertex(&self, index_count: u32, instance_count: u32, first_index: u32, base_vertex: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `drawIndexed()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.drawIndexed($1, $2, $3, $4, $5)"]
pub fn draw_indexed_with_instance_count_and_first_index_and_base_vertex_and_first_instance(&self, index_count: u32, instance_count: u32, first_index: u32, base_vertex: i32, first_instance: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `drawIndexedIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexedIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.drawIndexedIndirect($1, $2)"]
pub fn draw_indexed_indirect_with_u32(&self, indirect_buffer: &GpuBuffer, indirect_offset: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `drawIndexedIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexedIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.drawIndexedIndirect($1, $2)"]
pub fn draw_indexed_indirect_with_f64(&self, indirect_buffer: &GpuBuffer, indirect_offset: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `drawIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.drawIndirect($1, $2)"]
pub fn draw_indirect_with_u32(&self, indirect_buffer: &GpuBuffer, indirect_offset: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `drawIndirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.drawIndirect($1, $2)"]
pub fn draw_indirect_with_f64(&self, indirect_buffer: &GpuBuffer, indirect_offset: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuIndexFormat",))]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuIndexFormat`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setIndexBuffer($1, $2)"]
pub fn set_index_buffer(&self, buffer: &GpuBuffer, index_format: GpuIndexFormat) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuIndexFormat",))]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuIndexFormat`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setIndexBuffer($1, $2, $3)"]
pub fn set_index_buffer_with_u32(&self, buffer: &GpuBuffer, index_format: GpuIndexFormat, offset: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuIndexFormat",))]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuIndexFormat`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setIndexBuffer($1, $2, $3)"]
pub fn set_index_buffer_with_f64(&self, buffer: &GpuBuffer, index_format: GpuIndexFormat, offset: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuIndexFormat",))]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuIndexFormat`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setIndexBuffer($1, $2, $3, $4)"]
pub fn set_index_buffer_with_u32_and_u32(&self, buffer: &GpuBuffer, index_format: GpuIndexFormat, offset: u32, size: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuIndexFormat",))]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuIndexFormat`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setIndexBuffer($1, $2, $3, $4)"]
pub fn set_index_buffer_with_f64_and_u32(&self, buffer: &GpuBuffer, index_format: GpuIndexFormat, offset: f64, size: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuIndexFormat",))]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuIndexFormat`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setIndexBuffer($1, $2, $3, $4)"]
pub fn set_index_buffer_with_u32_and_f64(&self, buffer: &GpuBuffer, index_format: GpuIndexFormat, offset: u32, size: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuIndexFormat",))]
    #[doc = "The `setIndexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuIndexFormat`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setIndexBuffer($1, $2, $3, $4)"]
pub fn set_index_buffer_with_f64_and_f64(&self, buffer: &GpuBuffer, index_format: GpuIndexFormat, offset: f64, size: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPipeline")]
    #[doc = "The `setPipeline()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setPipeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuRenderPassEncoder`, `GpuRenderPipeline`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setPipeline($1)"]
pub fn set_pipeline(&self, pipeline: &GpuRenderPipeline) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setVertexBuffer($1, $2)"]
pub fn set_vertex_buffer(&self, slot: u32, buffer: Option<&GpuBuffer>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setVertexBuffer($1, $2, $3)"]
pub fn set_vertex_buffer_with_u32(&self, slot: u32, buffer: Option<&GpuBuffer>, offset: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setVertexBuffer($1, $2, $3)"]
pub fn set_vertex_buffer_with_f64(&self, slot: u32, buffer: Option<&GpuBuffer>, offset: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setVertexBuffer($1, $2, $3, $4)"]
pub fn set_vertex_buffer_with_u32_and_u32(&self, slot: u32, buffer: Option<&GpuBuffer>, offset: u32, size: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setVertexBuffer($1, $2, $3, $4)"]
pub fn set_vertex_buffer_with_f64_and_u32(&self, slot: u32, buffer: Option<&GpuBuffer>, offset: f64, size: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setVertexBuffer($1, $2, $3, $4)"]
pub fn set_vertex_buffer_with_u32_and_f64(&self, slot: u32, buffer: Option<&GpuBuffer>, offset: u32, size: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `setVertexBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuRenderPassEncoder {
    #[js::import = "$0.setVertexBuffer($1, $2, $3, $4)"]
pub fn set_vertex_buffer_with_f64_and_f64(&self, slot: u32, buffer: Option<&GpuBuffer>, offset: f64, size: f64) { panic!("web-sys intrinsic") }
}
