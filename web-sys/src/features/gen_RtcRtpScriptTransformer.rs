#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpScriptTransformer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransformer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcRtpScriptTransformer(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `RtcRtpScriptTransformer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransformer {
    #[js::import = "$0.readable"]
pub fn readable(&self) -> ReadableStream { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransformer`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransformer {
    #[js::import = "$0.writable"]
pub fn writable(&self) -> WritableStream { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onkeyframerequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/onkeyframerequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransformer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransformer {
    #[js::import = "$0.onkeyframerequest"]
pub fn onkeyframerequest(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onkeyframerequest` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/onkeyframerequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransformer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransformer {
    #[js::import = "$0.onkeyframerequest = $1"]
pub fn set_onkeyframerequest(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `options` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/options)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransformer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransformer {
    #[js::import = "$0.options"]
pub fn options(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `generateKeyFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/generateKeyFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransformer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransformer {
    #[js::import = "$0.generateKeyFrame()"]
pub fn generate_key_frame(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `generateKeyFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/generateKeyFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransformer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransformer {
    #[js::import = "$0.generateKeyFrame($1)"]
pub fn generate_key_frame_with_rid(&self, rid: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `sendKeyFrameRequest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCRtpScriptTransformer/sendKeyFrameRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpScriptTransformer`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl RtcRtpScriptTransformer {
    #[js::import = "$0.sendKeyFrameRequest()"]
pub fn send_key_frame_request(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
