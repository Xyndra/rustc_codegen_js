#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuPipelineError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuPipelineError(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineError`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuPipelineError {
    #[js::import = "$0.reason"]
pub fn reason(&self) -> GpuPipelineErrorReason { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorInit")]
    #[doc = "The `new GpuPipelineError(..)` constructor, creating a new instance of `GpuPipelineError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError/GPUPipelineError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineError`, `GpuPipelineErrorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuPipelineError {
    #[js::import = "new GPUPipelineError($0)"]
pub fn new(options: &GpuPipelineErrorInit) -> Result<GpuPipelineError, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorInit")]
    #[doc = "The `new GpuPipelineError(..)` constructor, creating a new instance of `GpuPipelineError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError/GPUPipelineError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineError`, `GpuPipelineErrorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuPipelineError {
    #[js::import = "new GPUPipelineError($0, $1)"]
pub fn new_with_message(message: &str, options: &GpuPipelineErrorInit,) -> Result<GpuPipelineError, JsValue> { panic!("web-sys intrinsic") }
}
