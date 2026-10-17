#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebTransportError(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorSource")]
    #[doc = "Getter for the `source` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/source)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`, `WebTransportErrorSource`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportError {
    #[js::import = "$0.source"]
pub fn source(&self) -> WebTransportErrorSource { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `streamErrorCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/streamErrorCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportError {
    #[js::import = "$0.streamErrorCode"]
pub fn stream_error_code(&self) -> Option<u8> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportError {
    #[js::import = "new WebTransportError()"]
pub fn new() -> Result<WebTransportError, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportError {
    #[js::import = "new WebTransportError($0)"]
pub fn new_with_message(message: &str) -> Result<WebTransportError, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WebTransportErrorOptions")]
    #[doc = "The `new WebTransportError(..)` constructor, creating a new instance of `WebTransportError`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportError/WebTransportError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportError`, `WebTransportErrorOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebTransportError {
    #[js::import = "new WebTransportError($0, $1)"]
pub fn new_with_message_and_options(message: &str, options: &WebTransportErrorOptions,) -> Result<WebTransportError, JsValue> { panic!("web-sys intrinsic") }
}
