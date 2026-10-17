#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuTexture` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuTexture(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.width"]
pub fn width(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.height"]
pub fn height(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `depthOrArrayLayers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/depthOrArrayLayers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.depthOrArrayLayers"]
pub fn depth_or_array_layers(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `mipLevelCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/mipLevelCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.mipLevelCount"]
pub fn mip_level_count(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `sampleCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/sampleCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.sampleCount"]
pub fn sample_count(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureDimension")]
    #[doc = "Getter for the `dimension` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/dimension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureDimension`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.dimension"]
pub fn dimension(&self) -> GpuTextureDimension { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Getter for the `format` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/format)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.format"]
pub fn format(&self) -> GpuTextureFormat { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `usage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/usage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.usage"]
pub fn usage(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.label"]
pub fn label(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `label` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/label)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.label = $1"]
pub fn set_label(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureView")]
    #[doc = "The `createView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureView`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.createView()"]
pub fn create_view(&self) -> Result<GpuTextureView, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "GpuTextureView", feature = "GpuTextureViewDescriptor",))]
    #[doc = "The `createView()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/createView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`, `GpuTextureView`, `GpuTextureViewDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.createView($1)"]
pub fn create_view_with_descriptor(&self, descriptor: &GpuTextureViewDescriptor) -> Result<GpuTextureView, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `destroy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUTexture/destroy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuTexture {
    #[js::import = "$0.destroy()"]
pub fn destroy(&self) { panic!("web-sys intrinsic") }
}
