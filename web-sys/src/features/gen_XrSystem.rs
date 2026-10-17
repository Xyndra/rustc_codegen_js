#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrSystem` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct XrSystem(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `ondevicechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/ondevicechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrSystem {
    #[js::import = "$0.ondevicechange"]
pub fn ondevicechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `ondevicechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/ondevicechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrSystem {
    #[js::import = "$0.ondevicechange = $1"]
pub fn set_ondevicechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSessionMode")]
    #[doc = "The `isSessionSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/isSessionSupported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionMode`, `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrSystem {
    #[js::import = "$0.isSessionSupported($1)"]
pub fn is_session_supported(&self, mode: XrSessionMode) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "XrSessionMode")]
    #[doc = "The `requestSession()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/requestSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionMode`, `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrSystem {
    #[js::import = "$0.requestSession($1)"]
pub fn request_session(&self, mode: XrSessionMode) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "XrSessionInit", feature = "XrSessionMode",))]
    #[doc = "The `requestSession()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRSystem/requestSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrSessionInit`, `XrSessionMode`, `XrSystem`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl XrSystem {
    #[js::import = "$0.requestSession($1, $2)"]
pub fn request_session_with_options(&self, mode: XrSessionMode, options: &XrSessionInit) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
