#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuAdapter` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct GpuAdapter(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSupportedFeatures")]
    #[doc = "Getter for the `features` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/features)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuSupportedFeatures`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapter {
    #[js::import = "$0.features"]
pub fn features(&self) -> GpuSupportedFeatures { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuSupportedLimits")]
    #[doc = "Getter for the `limits` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/limits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuSupportedLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapter {
    #[js::import = "$0.limits"]
pub fn limits(&self) -> GpuSupportedLimits { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuAdapterInfo")]
    #[doc = "Getter for the `info` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/info)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuAdapterInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapter {
    #[js::import = "$0.info"]
pub fn info(&self) -> GpuAdapterInfo { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `isFallbackAdapter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/isFallbackAdapter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapter {
    #[js::import = "$0.isFallbackAdapter"]
pub fn is_fallback_adapter(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapter {
    #[js::import = "$0.requestDevice()"]
pub fn request_device(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuDeviceDescriptor")]
    #[doc = "The `requestDevice()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapter/requestDevice)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapter`, `GpuDeviceDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GpuAdapter {
    #[js::import = "$0.requestDevice($1)"]
pub fn request_device_with_descriptor(&self, descriptor: &GpuDeviceDescriptor) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
