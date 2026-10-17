#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuSupportedLimits` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuSupportedLimits(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxTextureDimension1D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureDimension1D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxTextureDimension1D"]
pub fn max_texture_dimension_1d(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxTextureDimension2D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureDimension2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxTextureDimension2D"]
pub fn max_texture_dimension_2d(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxTextureDimension3D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureDimension3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxTextureDimension3D"]
pub fn max_texture_dimension_3d(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxTextureArrayLayers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxTextureArrayLayers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxTextureArrayLayers"]
pub fn max_texture_array_layers(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxBindGroups` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBindGroups)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxBindGroups"]
pub fn max_bind_groups(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxBindGroupsPlusVertexBuffers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBindGroupsPlusVertexBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxBindGroupsPlusVertexBuffers"]
pub fn max_bind_groups_plus_vertex_buffers(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxBindingsPerBindGroup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBindingsPerBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxBindingsPerBindGroup"]
pub fn max_bindings_per_bind_group(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxDynamicUniformBuffersPerPipelineLayout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxDynamicUniformBuffersPerPipelineLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxDynamicUniformBuffersPerPipelineLayout"]
pub fn max_dynamic_uniform_buffers_per_pipeline_layout(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxDynamicStorageBuffersPerPipelineLayout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxDynamicStorageBuffersPerPipelineLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxDynamicStorageBuffersPerPipelineLayout"]
pub fn max_dynamic_storage_buffers_per_pipeline_layout(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxSampledTexturesPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxSampledTexturesPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxSampledTexturesPerShaderStage"]
pub fn max_sampled_textures_per_shader_stage(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxSamplersPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxSamplersPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxSamplersPerShaderStage"]
pub fn max_samplers_per_shader_stage(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxStorageBuffersPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageBuffersPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxStorageBuffersPerShaderStage"]
pub fn max_storage_buffers_per_shader_stage(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxStorageTexturesPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageTexturesPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxStorageTexturesPerShaderStage"]
pub fn max_storage_textures_per_shader_stage(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxUniformBuffersPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxUniformBuffersPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxUniformBuffersPerShaderStage"]
pub fn max_uniform_buffers_per_shader_stage(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxUniformBufferBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxUniformBufferBindingSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxUniformBufferBindingSize"]
pub fn max_uniform_buffer_binding_size(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxStorageBufferBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxStorageBufferBindingSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxStorageBufferBindingSize"]
pub fn max_storage_buffer_binding_size(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `minUniformBufferOffsetAlignment` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/minUniformBufferOffsetAlignment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.minUniformBufferOffsetAlignment"]
pub fn min_uniform_buffer_offset_alignment(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `minStorageBufferOffsetAlignment` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/minStorageBufferOffsetAlignment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.minStorageBufferOffsetAlignment"]
pub fn min_storage_buffer_offset_alignment(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxVertexBuffers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxVertexBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxVertexBuffers"]
pub fn max_vertex_buffers(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxBufferSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxBufferSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxBufferSize"]
pub fn max_buffer_size(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxVertexAttributes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxVertexAttributes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxVertexAttributes"]
pub fn max_vertex_attributes(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxVertexBufferArrayStride` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxVertexBufferArrayStride)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxVertexBufferArrayStride"]
pub fn max_vertex_buffer_array_stride(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxInterStageShaderVariables` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxInterStageShaderVariables)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxInterStageShaderVariables"]
pub fn max_inter_stage_shader_variables(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxColorAttachments` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxColorAttachments)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxColorAttachments"]
pub fn max_color_attachments(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxColorAttachmentBytesPerSample` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxColorAttachmentBytesPerSample)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxColorAttachmentBytesPerSample"]
pub fn max_color_attachment_bytes_per_sample(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxComputeWorkgroupStorageSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupStorageSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxComputeWorkgroupStorageSize"]
pub fn max_compute_workgroup_storage_size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxComputeInvocationsPerWorkgroup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeInvocationsPerWorkgroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxComputeInvocationsPerWorkgroup"]
pub fn max_compute_invocations_per_workgroup(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxComputeWorkgroupSizeX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupSizeX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxComputeWorkgroupSizeX"]
pub fn max_compute_workgroup_size_x(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxComputeWorkgroupSizeY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupSizeY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxComputeWorkgroupSizeY"]
pub fn max_compute_workgroup_size_y(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxComputeWorkgroupSizeZ` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupSizeZ)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxComputeWorkgroupSizeZ"]
pub fn max_compute_workgroup_size_z(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `maxComputeWorkgroupsPerDimension` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUSupportedLimits/maxComputeWorkgroupsPerDimension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuSupportedLimits {
    #[js::import = "$0.maxComputeWorkgroupsPerDimension"]
pub fn max_compute_workgroups_per_dimension(&self) -> u32 { panic!("web-sys intrinsic") }
}
