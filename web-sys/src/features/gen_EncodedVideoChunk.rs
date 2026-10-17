#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedVideoChunk` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct EncodedVideoChunk(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedVideoChunkType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`, `EncodedVideoChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedVideoChunk {
    #[js::import = "$0.type"]
pub fn type_(&self) -> EncodedVideoChunkType { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedVideoChunk {
    #[js::import = "$0.timestamp"]
pub fn timestamp(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedVideoChunk {
    #[js::import = "$0.duration"]
pub fn duration(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `byteLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/byteLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedVideoChunk {
    #[js::import = "$0.byteLength"]
pub fn byte_length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedVideoChunkInit")]
    #[doc = "The `new EncodedVideoChunk(..)` constructor, creating a new instance of `EncodedVideoChunk`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/EncodedVideoChunk)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`, `EncodedVideoChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedVideoChunk {
    #[js::import = "new EncodedVideoChunk($0)"]
pub fn new(init: &EncodedVideoChunkInit) -> Result<EncodedVideoChunk, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedVideoChunk {
    #[js::import = "$0.copyTo($1)"]
pub fn copy_to_with_buffer_source(&self, destination: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedVideoChunk {
    #[js::import = "$0.copyTo($1)"]
pub fn copy_to_with_u8_slice(&self, destination: &mut [u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedVideoChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedVideoChunk {
    #[js::import = "$0.copyTo($1)"]
pub fn copy_to_with_u8_array(&self, destination: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
