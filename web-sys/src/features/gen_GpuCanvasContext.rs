#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuCanvasContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuCanvasContext(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCanvasContext {
    #[js::import = "$0.canvas"]
pub fn canvas(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasConfiguration")]
    #[doc = "The `configure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/configure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`, `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCanvasContext {
    #[js::import = "$0.configure($1)"]
pub fn configure(&self, configuration: &GpuCanvasConfiguration) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuCanvasConfiguration")]
    #[doc = "The `getConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/getConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasConfiguration`, `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCanvasContext {
    #[js::import = "$0.getConfiguration()"]
pub fn get_configuration(&self) -> Option<GpuCanvasConfiguration> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[doc = "The `getCurrentTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/getCurrentTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasContext`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCanvasContext {
    #[js::import = "$0.getCurrentTexture()"]
pub fn get_current_texture(&self) -> Result<GpuTexture, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `unconfigure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUCanvasContext/unconfigure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuCanvasContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuCanvasContext {
    #[js::import = "$0.unconfigure()"]
pub fn unconfigure(&self) { panic!("web-sys intrinsic") }
}
