#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Gpu` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct Gpu(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WgslLanguageFeatures")]
    #[doc = "Getter for the `wgslLanguageFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/wgslLanguageFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `WgslLanguageFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Gpu {
    #[js::import = "$0.wgslLanguageFeatures"]
pub fn wgsl_language_features(&self) -> WgslLanguageFeatures { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "The `getPreferredCanvasFormat()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/getPreferredCanvasFormat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Gpu {
    #[js::import = "$0.getPreferredCanvasFormat()"]
pub fn get_preferred_canvas_format(&self) -> GpuTextureFormat { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `requestAdapter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Gpu {
    #[js::import = "$0.requestAdapter()"]
pub fn request_adapter(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuRequestAdapterOptions")]
    #[doc = "The `requestAdapter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPU/requestAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gpu`, `GpuRequestAdapterOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Gpu {
    #[js::import = "$0.requestAdapter($1)"]
pub fn request_adapter_with_options(&self, options: &GpuRequestAdapterOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
