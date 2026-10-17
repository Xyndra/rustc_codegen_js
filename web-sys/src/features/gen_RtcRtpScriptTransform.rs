#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpScriptTransform` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpScriptTransform(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Worker")]
    #[doc = "The `new RtcRtpScriptTransform(..)` constructor, creating a new instance of `RtcRtpScriptTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransform/RTCRtpScriptTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`, `Worker`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransform {
    #[js::import = "new RTCRtpScriptTransform($0)"]
pub fn new(worker: &Worker) -> Result<RtcRtpScriptTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Worker")]
    #[doc = "The `new RtcRtpScriptTransform(..)` constructor, creating a new instance of `RtcRtpScriptTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransform/RTCRtpScriptTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`, `Worker`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransform {
    #[js::import = "new RTCRtpScriptTransform($0, $1)"]
pub fn new_with_options(worker: &Worker, options: &::wasm_bindgen::JsValue,) -> Result<RtcRtpScriptTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Worker")]
    #[doc = "The `new RtcRtpScriptTransform(..)` constructor, creating a new instance of `RtcRtpScriptTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransform/RTCRtpScriptTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransform`, `Worker`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransform {
    #[js::import = "new RTCRtpScriptTransform($0, $1, $2)"]
pub fn new_with_options_and_transfer(worker: &Worker, options: &::wasm_bindgen::JsValue, transfer: &::wasm_bindgen::JsValue,) -> Result<RtcRtpScriptTransform, JsValue> { panic!("web-sys intrinsic") }
}
