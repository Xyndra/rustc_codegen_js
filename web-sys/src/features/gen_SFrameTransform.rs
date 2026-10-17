#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SFrameTransform` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct SFrameTransform(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransform {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransform {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransform {
    #[js::import = "$0.readable"]
pub fn readable(&self) -> ReadableStream { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransform {
    #[js::import = "$0.writable"]
pub fn writable(&self) -> WritableStream { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new SFrameTransform(..)` constructor, creating a new instance of `SFrameTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/SFrameTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransform {
    #[js::import = "new SFrameTransform()"]
pub fn new() -> Result<SFrameTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SFrameTransformOptions")]
    #[doc = "The `new SFrameTransform(..)` constructor, creating a new instance of `SFrameTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/SFrameTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SFrameTransform`, `SFrameTransformOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransform {
    #[js::import = "new SFrameTransform($0)"]
pub fn new_with_options(options: &SFrameTransformOptions) -> Result<SFrameTransform, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CryptoKey")]
    #[doc = "The `setEncryptionKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/setEncryptionKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransform {
    #[js::import = "$0.setEncryptionKey($1)"]
pub fn set_encryption_key(&self, key: &CryptoKey) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CryptoKey")]
    #[doc = "The `setEncryptionKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/setEncryptionKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransform {
    #[js::import = "$0.setEncryptionKey($1, $2)"]
pub fn set_encryption_key_with_u32(&self, key: &CryptoKey, key_id: u32) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CryptoKey")]
    #[doc = "The `setEncryptionKey()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SFrameTransform/setEncryptionKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `SFrameTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl SFrameTransform {
    #[js::import = "$0.setEncryptionKey($1, $2)"]
pub fn set_encryption_key_with_f64(&self, key: &CryptoKey, key_id: f64) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
