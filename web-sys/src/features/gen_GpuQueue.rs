#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuQueue` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuQueue(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.label = $1"]
pub fn set_label(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuCopyExternalImageDestInfo",
        feature = "GpuCopyExternalImageSourceInfo",
    ))]
    #[doc = "The `copyExternalImageToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyExternalImageToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageDestInfo`, `GpuCopyExternalImageSourceInfo`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.copyExternalImageToTexture($1, $2, $3)"]
pub fn copy_external_image_to_texture_with_u32_sequence(&self, source: &GpuCopyExternalImageSourceInfo, destination: &GpuCopyExternalImageDestInfo, copy_size: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuCopyExternalImageDestInfo",
        feature = "GpuCopyExternalImageSourceInfo",
        feature = "GpuExtent3dDict",
    ))]
    #[doc = "The `copyExternalImageToTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/copyExternalImageToTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCopyExternalImageDestInfo`, `GpuCopyExternalImageSourceInfo`, `GpuExtent3dDict`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.copyExternalImageToTexture($1, $2, $3)"]
pub fn copy_external_image_to_texture_with_gpu_extent_3d_dict(&self, source: &GpuCopyExternalImageSourceInfo, destination: &GpuCopyExternalImageDestInfo, copy_size: &GpuExtent3dDict) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `onSubmittedWorkDone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/onSubmittedWorkDone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.onSubmittedWorkDone()"]
pub fn on_submitted_work_done(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `submit()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/submit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.submit($1)"]
pub fn submit(&self, command_buffers: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3)"]
pub fn write_buffer_with_u32_and_buffer_source(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3)"]
pub fn write_buffer_with_f64_and_buffer_source(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3)"]
pub fn write_buffer_with_u32_and_u8_slice(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &[u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3)"]
pub fn write_buffer_with_f64_and_u8_slice(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &[u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3)"]
pub fn write_buffer_with_u32_and_u8_array(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3)"]
pub fn write_buffer_with_f64_and_u8_array(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_u32_and_buffer_source_and_u32(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Object, data_offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_f64_and_buffer_source_and_u32(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Object, data_offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_u32_and_u8_slice_and_u32(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &[u8], data_offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_f64_and_u8_slice_and_u32(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &[u8], data_offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_u32_and_u8_array_and_u32(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Uint8Array, data_offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_f64_and_u8_array_and_u32(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Uint8Array, data_offset: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_u32_and_buffer_source_and_f64(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Object, data_offset: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_f64_and_buffer_source_and_f64(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Object, data_offset: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_u32_and_u8_slice_and_f64(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &[u8], data_offset: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_f64_and_u8_slice_and_f64(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &[u8], data_offset: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_u32_and_u8_array_and_f64(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Uint8Array, data_offset: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4)"]
pub fn write_buffer_with_f64_and_u8_array_and_f64(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Uint8Array, data_offset: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_buffer_source_and_u32_and_u32(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Object, data_offset: u32, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_buffer_source_and_u32_and_u32(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Object, data_offset: u32, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_u8_slice_and_u32_and_u32(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &[u8], data_offset: u32, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_u8_slice_and_u32_and_u32(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &[u8], data_offset: u32, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_u8_array_and_u32_and_u32(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Uint8Array, data_offset: u32, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_u8_array_and_u32_and_u32(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Uint8Array, data_offset: u32, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_buffer_source_and_f64_and_u32(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Object, data_offset: f64, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_buffer_source_and_f64_and_u32(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Object, data_offset: f64, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_u8_slice_and_f64_and_u32(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &[u8], data_offset: f64, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_u8_slice_and_f64_and_u32(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &[u8], data_offset: f64, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_u8_array_and_f64_and_u32(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Uint8Array, data_offset: f64, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_u8_array_and_f64_and_u32(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Uint8Array, data_offset: f64, size: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_buffer_source_and_u32_and_f64(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Object, data_offset: u32, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_buffer_source_and_u32_and_f64(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Object, data_offset: u32, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_u8_slice_and_u32_and_f64(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &[u8], data_offset: u32, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_u8_slice_and_u32_and_f64(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &[u8], data_offset: u32, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_u8_array_and_u32_and_f64(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Uint8Array, data_offset: u32, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_u8_array_and_u32_and_f64(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Uint8Array, data_offset: u32, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_buffer_source_and_f64_and_f64(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Object, data_offset: f64, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_buffer_source_and_f64_and_f64(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Object, data_offset: f64, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_u8_slice_and_f64_and_f64(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &[u8], data_offset: f64, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_u8_slice_and_f64_and_f64(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &[u8], data_offset: f64, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_u32_and_u8_array_and_f64_and_f64(&self, buffer: &GpuBuffer, buffer_offset: u32, data: &::js_sys::Uint8Array, data_offset: f64, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBuffer")]
    #[doc = "The `writeBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBuffer`, `GpuQueue`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeBuffer($1, $2, $3, $4, $5)"]
pub fn write_buffer_with_f64_and_u8_array_and_f64_and_f64(&self, buffer: &GpuBuffer, buffer_offset: f64, data: &::js_sys::Uint8Array, data_offset: f64, size: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeTexture($1, $2, $3, $4)"]
pub fn write_texture_with_buffer_source_and_u32_sequence(&self, destination: &GpuTexelCopyTextureInfo, data: &::js_sys::Object, data_layout: &GpuTexelCopyBufferLayout, size: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeTexture($1, $2, $3, $4)"]
pub fn write_texture_with_u8_slice_and_u32_sequence(&self, destination: &GpuTexelCopyTextureInfo, data: &[u8], data_layout: &GpuTexelCopyBufferLayout, size: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeTexture($1, $2, $3, $4)"]
pub fn write_texture_with_u8_array_and_u32_sequence(&self, destination: &GpuTexelCopyTextureInfo, data: &::js_sys::Uint8Array, data_layout: &GpuTexelCopyBufferLayout, size: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`, `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeTexture($1, $2, $3, $4)"]
pub fn write_texture_with_buffer_source_and_gpu_extent_3d_dict(&self, destination: &GpuTexelCopyTextureInfo, data: &::js_sys::Object, data_layout: &GpuTexelCopyBufferLayout, size: &GpuExtent3dDict) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`, `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeTexture($1, $2, $3, $4)"]
pub fn write_texture_with_u8_slice_and_gpu_extent_3d_dict(&self, destination: &GpuTexelCopyTextureInfo, data: &[u8], data_layout: &GpuTexelCopyBufferLayout, size: &GpuExtent3dDict) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GpuExtent3dDict",
        feature = "GpuTexelCopyBufferLayout",
        feature = "GpuTexelCopyTextureInfo",
    ))]
    #[doc = "The `writeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUQueue/writeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`, `GpuQueue`, `GpuTexelCopyBufferLayout`, `GpuTexelCopyTextureInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuQueue {
    #[js::import = "$0.writeTexture($1, $2, $3, $4)"]
pub fn write_texture_with_u8_array_and_gpu_extent_3d_dict(&self, destination: &GpuTexelCopyTextureInfo, data: &::js_sys::Uint8Array, data_layout: &GpuTexelCopyBufferLayout, size: &GpuExtent3dDict) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
