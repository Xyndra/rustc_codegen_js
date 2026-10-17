#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoDecoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct VideoDecoder(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CodecState")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CodecState`, `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoDecoder {
    #[js::import = "$0.state"]
pub fn state(&self) -> CodecState { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `decodeQueueSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/decodeQueueSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoDecoder {
    #[js::import = "$0.decodeQueueSize"]
pub fn decode_queue_size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderInit")]
    #[doc = "The `new VideoDecoder(..)` constructor, creating a new instance of `VideoDecoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/VideoDecoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`, `VideoDecoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoDecoder {
    #[js::import = "new VideoDecoder($0)"]
pub fn new(init: &VideoDecoderInit) -> Result<VideoDecoder, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoDecoder {
    #[js::import = "$0.close()"]
pub fn close(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "The `configure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/configure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoDecoder {
    #[js::import = "$0.configure($1)"]
pub fn configure(&self, config: &VideoDecoderConfig) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "EncodedVideoChunk")]
    #[doc = "The `decode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/decode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunk`, `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoDecoder {
    #[js::import = "$0.decode($1)"]
pub fn decode(&self, chunk: &EncodedVideoChunk) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoDecoder {
    #[js::import = "$0.flush()"]
pub fn flush(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "The `isConfigSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/isConfigSupported_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoDecoder {
    #[js::import = "VideoDecoder.isConfigSupported"]
pub fn is_config_supported(config: &VideoDecoderConfig) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoDecoder/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoDecoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoDecoder {
    #[js::import = "$0.reset()"]
pub fn reset(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
