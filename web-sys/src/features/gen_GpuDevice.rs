#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuDevice` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuDevice(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSupportedFeatures")]
    #[doc = "Getter for the `features` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/features)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.features"]
pub fn features(&self) -> GpuSupportedFeatures { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSupportedLimits")]
    #[doc = "Getter for the `limits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/limits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.limits"]
pub fn limits(&self) -> GpuSupportedLimits { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAdapterInfo")]
    #[doc = "Getter for the `adapterInfo` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/adapterInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterInfo`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.adapterInfo"]
pub fn adapter_info(&self) -> GpuAdapterInfo { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuQueue")]
    #[doc = "Getter for the `queue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/queue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.queue"]
pub fn queue(&self) -> GpuQueue { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `lost` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/lost)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.lost"]
pub fn lost(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onuncapturederror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.onuncapturederror"]
pub fn onuncapturederror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onuncapturederror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.onuncapturederror = $1"]
pub fn set_onuncapturederror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.label = $1"]
pub fn set_label(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBindGroup", feature = "GpuBindGroupDescriptor",))]
    #[doc = "The `createBindGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroup`, `GpuBindGroupDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createBindGroup($1)"]
pub fn create_bind_group(&self, descriptor: &GpuBindGroupDescriptor) -> GpuBindGroup { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuBindGroupLayout",
        feature = "GpuBindGroupLayoutDescriptor",
    ))]
    #[doc = "The `createBindGroupLayout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroupLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupLayout`, `GpuBindGroupLayoutDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createBindGroupLayout($1)"]
pub fn create_bind_group_layout(&self, descriptor: &GpuBindGroupLayoutDescriptor) -> Result<GpuBindGroupLayout, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuBufferDescriptor",))]
    #[doc = "The `createBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuBufferDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createBuffer($1)"]
pub fn create_buffer(&self, descriptor: &GpuBufferDescriptor) -> Result<GpuBuffer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCommandEncoder")]
    #[doc = "The `createCommandEncoder()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createCommandEncoder()"]
pub fn create_command_encoder(&self) -> GpuCommandEncoder { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuCommandEncoder", feature = "GpuCommandEncoderDescriptor",))]
    #[doc = "The `createCommandEncoder()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuCommandEncoderDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createCommandEncoder($1)"]
pub fn create_command_encoder_with_descriptor(&self, descriptor: &GpuCommandEncoderDescriptor) -> GpuCommandEncoder { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuComputePipeline",
        feature = "GpuComputePipelineDescriptor",
    ))]
    #[doc = "The `createComputePipeline()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePipeline`, `GpuComputePipelineDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createComputePipeline($1)"]
pub fn create_compute_pipeline(&self, descriptor: &GpuComputePipelineDescriptor) -> GpuComputePipeline { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuComputePipelineDescriptor")]
    #[doc = "The `createComputePipelineAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipelineAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuComputePipelineDescriptor`, `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createComputePipelineAsync($1)"]
pub fn create_compute_pipeline_async(&self, descriptor: &GpuComputePipelineDescriptor) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuPipelineLayout", feature = "GpuPipelineLayoutDescriptor",))]
    #[doc = "The `createPipelineLayout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createPipelineLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuPipelineLayout`, `GpuPipelineLayoutDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createPipelineLayout($1)"]
pub fn create_pipeline_layout(&self, descriptor: &GpuPipelineLayoutDescriptor) -> GpuPipelineLayout { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuQuerySet", feature = "GpuQuerySetDescriptor",))]
    #[doc = "The `createQuerySet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createQuerySet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuQuerySet`, `GpuQuerySetDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createQuerySet($1)"]
pub fn create_query_set(&self, descriptor: &GpuQuerySetDescriptor) -> Result<GpuQuerySet, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuRenderBundleEncoder",
        feature = "GpuRenderBundleEncoderDescriptor",
    ))]
    #[doc = "The `createRenderBundleEncoder()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderBundleEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderBundleEncoder`, `GpuRenderBundleEncoderDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createRenderBundleEncoder($1)"]
pub fn create_render_bundle_encoder(&self, descriptor: &GpuRenderBundleEncoderDescriptor) -> Result<GpuRenderBundleEncoder, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuRenderPipeline", feature = "GpuRenderPipelineDescriptor",))]
    #[doc = "The `createRenderPipeline()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderPipeline`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createRenderPipeline($1)"]
pub fn create_render_pipeline(&self, descriptor: &GpuRenderPipelineDescriptor) -> Result<GpuRenderPipeline, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRenderPipelineDescriptor")]
    #[doc = "The `createRenderPipelineAsync()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipelineAsync)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuRenderPipelineDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createRenderPipelineAsync($1)"]
pub fn create_render_pipeline_async(&self, descriptor: &GpuRenderPipelineDescriptor) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSampler")]
    #[doc = "The `createSampler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuSampler`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createSampler()"]
pub fn create_sampler(&self) -> GpuSampler { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuSampler", feature = "GpuSamplerDescriptor",))]
    #[doc = "The `createSampler()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuSampler`, `GpuSamplerDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createSampler($1)"]
pub fn create_sampler_with_descriptor(&self, descriptor: &GpuSamplerDescriptor) -> GpuSampler { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuShaderModule", feature = "GpuShaderModuleDescriptor",))]
    #[doc = "The `createShaderModule()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createShaderModule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuShaderModule`, `GpuShaderModuleDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createShaderModule($1)"]
pub fn create_shader_module(&self, descriptor: &GpuShaderModuleDescriptor) -> GpuShaderModule { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuTexture", feature = "GpuTextureDescriptor",))]
    #[doc = "The `createTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuTexture`, `GpuTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.createTexture($1)"]
pub fn create_texture(&self, descriptor: &GpuTextureDescriptor) -> Result<GpuTexture, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `destroy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/destroy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.destroy()"]
pub fn destroy(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExternalTexture",
        feature = "GpuExternalTextureDescriptor",
    ))]
    #[doc = "The `importExternalTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/importExternalTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuExternalTexture`, `GpuExternalTextureDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.importExternalTexture($1)"]
pub fn import_external_texture(&self, descriptor: &GpuExternalTextureDescriptor) -> Result<GpuExternalTexture, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `popErrorScope()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/popErrorScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.popErrorScope()"]
pub fn pop_error_scope(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuErrorFilter")]
    #[doc = "The `pushErrorScope()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/pushErrorScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuDevice`, `GpuErrorFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuDevice {
    #[js::import = "$0.pushErrorScope($1)"]
pub fn push_error_scope(&self, filter: GpuErrorFilter) { panic!("web-sys intrinsic") }
}
