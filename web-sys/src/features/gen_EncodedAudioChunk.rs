#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedAudioChunk` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct EncodedAudioChunk(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedAudioChunkType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`, `EncodedAudioChunkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedAudioChunk {
    #[js::import = "$0.type"]
pub fn type_(&self) -> EncodedAudioChunkType { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedAudioChunk {
    #[js::import = "$0.timestamp"]
pub fn timestamp(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedAudioChunk {
    #[js::import = "$0.duration"]
pub fn duration(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `byteLength` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/byteLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedAudioChunk {
    #[js::import = "$0.byteLength"]
pub fn byte_length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedAudioChunkInit")]
    #[doc = "The `new EncodedAudioChunk(..)` constructor, creating a new instance of `EncodedAudioChunk`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/EncodedAudioChunk)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`, `EncodedAudioChunkInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedAudioChunk {
    #[js::import = "new EncodedAudioChunk($0)"]
pub fn new(init: &EncodedAudioChunkInit) -> Result<EncodedAudioChunk, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedAudioChunk {
    #[js::import = "$0.copyTo($1)"]
pub fn copy_to_with_buffer_source(&self, destination: &::js_sys::Object) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedAudioChunk {
    #[js::import = "$0.copyTo($1)"]
pub fn copy_to_with_u8_slice(&self, destination: &mut [u8]) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EncodedAudioChunk/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunk`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl EncodedAudioChunk {
    #[js::import = "$0.copyTo($1)"]
pub fn copy_to_with_u8_array(&self, destination: &::js_sys::Uint8Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
