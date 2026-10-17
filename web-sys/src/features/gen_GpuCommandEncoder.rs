#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCommandEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuCommandEncoder(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.label = $1"]
pub fn set_label(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuComputePassEncoder")]
    #[doc = "The `beginComputePass()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.beginComputePass()"]
pub fn begin_compute_pass(&self) -> GpuComputePassEncoder { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuComputePassDescriptor",
        feature = "GpuComputePassEncoder",
    ))]
    #[doc = "The `beginComputePass()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuComputePassDescriptor`, `GpuComputePassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.beginComputePass($1)"]
pub fn begin_compute_pass_with_descriptor(&self, descriptor: &GpuComputePassDescriptor) -> GpuComputePassEncoder { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuRenderPassDescriptor", feature = "GpuRenderPassEncoder",))]
    #[doc = "The `beginRenderPass()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginRenderPass)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuRenderPassDescriptor`, `GpuRenderPassEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.beginRenderPass($1)"]
pub fn begin_render_pass(&self, descriptor: &GpuRenderPassDescriptor) -> Result<GpuRenderPassEncoder, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `clearBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.clearBuffer($1)"]
pub fn clear_buffer(&self, buffer: &GpuBuffer) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `clearBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.clearBuffer($1, $2)"]
pub fn clear_buffer_with_u32(&self, buffer: &GpuBuffer, offset: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `clearBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.clearBuffer($1, $2)"]
pub fn clear_buffer_with_f64(&self, buffer: &GpuBuffer, offset: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `clearBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.clearBuffer($1, $2, $3)"]
pub fn clear_buffer_with_u32_and_u32(&self, buffer: &GpuBuffer, offset: u32, size: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `clearBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.clearBuffer($1, $2, $3)"]
pub fn clear_buffer_with_f64_and_u32(&self, buffer: &GpuBuffer, offset: f64, size: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `clearBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.clearBuffer($1, $2, $3)"]
pub fn clear_buffer_with_u32_and_f64(&self, buffer: &GpuBuffer, offset: u32, size: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `clearBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.clearBuffer($1, $2, $3)"]
pub fn clear_buffer_with_f64_and_f64(&self, buffer: &GpuBuffer, offset: f64, size: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4)"]
pub fn copy_buffer_to_buffer_with_u32_and_u32(&self, source: &GpuBuffer, source_offset: u32, destination: &GpuBuffer, destination_offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4)"]
pub fn copy_buffer_to_buffer_with_f64_and_u32(&self, source: &GpuBuffer, source_offset: f64, destination: &GpuBuffer, destination_offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4)"]
pub fn copy_buffer_to_buffer_with_u32_and_f64(&self, source: &GpuBuffer, source_offset: u32, destination: &GpuBuffer, destination_offset: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4)"]
pub fn copy_buffer_to_buffer_with_f64_and_f64(&self, source: &GpuBuffer, source_offset: f64, destination: &GpuBuffer, destination_offset: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4, $5)"]
pub fn copy_buffer_to_buffer_with_u32_and_u32_and_u32(&self, source: &GpuBuffer, source_offset: u32, destination: &GpuBuffer, destination_offset: u32, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4, $5)"]
pub fn copy_buffer_to_buffer_with_f64_and_u32_and_u32(&self, source: &GpuBuffer, source_offset: f64, destination: &GpuBuffer, destination_offset: u32, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4, $5)"]
pub fn copy_buffer_to_buffer_with_u32_and_f64_and_u32(&self, source: &GpuBuffer, source_offset: u32, destination: &GpuBuffer, destination_offset: f64, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4, $5)"]
pub fn copy_buffer_to_buffer_with_f64_and_f64_and_u32(&self, source: &GpuBuffer, source_offset: f64, destination: &GpuBuffer, destination_offset: f64, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4, $5)"]
pub fn copy_buffer_to_buffer_with_u32_and_u32_and_f64(&self, source: &GpuBuffer, source_offset: u32, destination: &GpuBuffer, destination_offset: u32, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4, $5)"]
pub fn copy_buffer_to_buffer_with_f64_and_u32_and_f64(&self, source: &GpuBuffer, source_offset: f64, destination: &GpuBuffer, destination_offset: u32, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4, $5)"]
pub fn copy_buffer_to_buffer_with_u32_and_f64_and_f64(&self, source: &GpuBuffer, source_offset: u32, destination: &GpuBuffer, destination_offset: f64, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `copyBufferToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToBuffer($1, $2, $3, $4, $5)"]
pub fn copy_buffer_to_buffer_with_f64_and_f64_and_f64(&self, source: &GpuBuffer, source_offset: f64, destination: &GpuBuffer, destination_offset: f64, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuTexelCopyBufferInfo",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `copyBufferToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuTexelCopyBufferInfo`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToTexture($1, $2, $3)"]
pub fn copy_buffer_to_texture_with_u32_sequence(&self, source: &GpuTexelCopyBufferInfo, destination: &GpuTexelCopyTextureInfo, copy_size: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuTexelCopyBufferInfo",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `copyBufferToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuTexelCopyBufferInfo`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyBufferToTexture($1, $2, $3)"]
pub fn copy_buffer_to_texture_with_gpu_extent_3d_dict(&self, source: &GpuTexelCopyBufferInfo, destination: &GpuTexelCopyTextureInfo, copy_size: &GpuExtent3dDict) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuTexelCopyBufferInfo",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `copyTextureToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuTexelCopyBufferInfo`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyTextureToBuffer($1, $2, $3)"]
pub fn copy_texture_to_buffer_with_u32_sequence(&self, source: &GpuTexelCopyTextureInfo, destination: &GpuTexelCopyBufferInfo, copy_size: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuTexelCopyBufferInfo",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `copyTextureToBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuTexelCopyBufferInfo`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyTextureToBuffer($1, $2, $3)"]
pub fn copy_texture_to_buffer_with_gpu_extent_3d_dict(&self, source: &GpuTexelCopyTextureInfo, destination: &GpuTexelCopyBufferInfo, copy_size: &GpuExtent3dDict) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexelCopyTextureInfo")]
    #[doc = "The `copyTextureToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyTextureToTexture($1, $2, $3)"]
pub fn copy_texture_to_texture_with_u32_sequence(&self, source: &GpuTexelCopyTextureInfo, destination: &GpuTexelCopyTextureInfo, copy_size: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuExtent3dDict", feature = "GpuTexelCopyTextureInfo",))]
    #[doc = "The `copyTextureToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`, `GpuExtent3dDict`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.copyTextureToTexture($1, $2, $3)"]
pub fn copy_texture_to_texture_with_gpu_extent_3d_dict(&self, source: &GpuTexelCopyTextureInfo, destination: &GpuTexelCopyTextureInfo, copy_size: &GpuExtent3dDict) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCommandBuffer")]
    #[doc = "The `finish()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandBuffer`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.finish()"]
pub fn finish(&self) -> GpuCommandBuffer { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuCommandBuffer", feature = "GpuCommandBufferDescriptor",))]
    #[doc = "The `finish()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandBuffer`, `GpuCommandBufferDescriptor`, `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.finish($1)"]
pub fn finish_with_descriptor(&self, descriptor: &GpuCommandBufferDescriptor) -> GpuCommandBuffer { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuQuerySet",))]
    #[doc = "The `resolveQuerySet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/resolveQuerySet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`, `GpuQuerySet`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.resolveQuerySet($1, $2, $3, $4, $5)"]
pub fn resolve_query_set_with_u32(&self, query_set: &GpuQuerySet, first_query: u32, query_count: u32, destination: &GpuBuffer, destination_offset: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuBuffer", feature = "GpuQuerySet",))]
    #[doc = "The `resolveQuerySet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/resolveQuerySet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuCommandEncoder`, `GpuQuerySet`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.resolveQuerySet($1, $2, $3, $4, $5)"]
pub fn resolve_query_set_with_f64(&self, query_set: &GpuQuerySet, first_query: u32, query_count: u32, destination: &GpuBuffer, destination_offset: f64) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `insertDebugMarker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/insertDebugMarker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.insertDebugMarker($1)"]
pub fn insert_debug_marker(&self, marker_label: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `popDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/popDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.popDebugGroup()"]
pub fn pop_debug_group(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `pushDebugGroup()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/pushDebugGroup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCommandEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCommandEncoder {
    #[js::import = "$0.pushDebugGroup($1)"]
pub fn push_debug_group(&self, group_label: &str) { panic!("web-sys intrinsic") }
}
