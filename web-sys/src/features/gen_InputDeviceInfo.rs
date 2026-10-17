#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputDeviceInfo` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputDeviceInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputDeviceInfo`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct InputDeviceInfo(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "MediaTrackCapabilities")]
    #[doc = "The `getCapabilities()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/InputDeviceInfo/getCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputDeviceInfo`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl InputDeviceInfo {
    #[js::import = "$0.getCapabilities()"]
pub fn get_capabilities(&self) -> MediaTrackCapabilities { panic!("web-sys intrinsic") }
}
