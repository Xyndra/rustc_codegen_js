#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuAdapterInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuAdapterInfo(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `vendor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/vendor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapterInfo {
    #[js::import = "$0.vendor"]
pub fn vendor(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `architecture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/architecture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapterInfo {
    #[js::import = "$0.architecture"]
pub fn architecture(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapterInfo {
    #[js::import = "$0.device"]
pub fn device(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `description` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterInfo/description)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapterInfo {
    #[js::import = "$0.description"]
pub fn description(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
