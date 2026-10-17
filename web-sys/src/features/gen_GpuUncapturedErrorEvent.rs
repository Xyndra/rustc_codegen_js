#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuUncapturedErrorEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUUncapturedErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuUncapturedErrorEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuUncapturedErrorEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuError")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUUncapturedErrorEvent/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuError`, `GpuUncapturedErrorEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuUncapturedErrorEvent {
    #[js::import = "$0.error"]
pub fn error(&self) -> GpuError { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuUncapturedErrorEventInit")]
    #[doc = "The `new GpuUncapturedErrorEvent(..)` constructor, creating a new instance of `GpuUncapturedErrorEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUUncapturedErrorEvent/GPUUncapturedErrorEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuUncapturedErrorEvent`, `GpuUncapturedErrorEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuUncapturedErrorEvent {
    #[js::import = "new GPUUncapturedErrorEvent($0, $1)"]
pub fn new(type_: &str, gpu_uncaptured_error_event_init_dict: &GpuUncapturedErrorEventInit,) -> Result<GpuUncapturedErrorEvent, JsValue> { panic!("web-sys intrinsic") }
}
