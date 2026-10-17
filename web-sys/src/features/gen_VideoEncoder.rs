#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoEncoder` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct VideoEncoder(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CodecState")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CodecState`, `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "$0.state"]
pub fn state(&self) -> CodecState { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `encodeQueueSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encodeQueueSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "$0.encodeQueueSize"]
pub fn encode_queue_size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderInit")]
    #[doc = "The `new VideoEncoder(..)` constructor, creating a new instance of `VideoEncoder`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/VideoEncoder)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoEncoderInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "new VideoEncoder($0)"]
pub fn new(init: &VideoEncoderInit) -> Result<VideoEncoder, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "$0.close()"]
pub fn close(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderConfig")]
    #[doc = "The `configure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/configure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "$0.configure($1)"]
pub fn configure(&self, config: &VideoEncoderConfig) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `encode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "$0.encode($1)"]
pub fn encode(&self, frame: &VideoFrame) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "VideoEncoderEncodeOptions", feature = "VideoFrame",))]
    #[doc = "The `encode()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/encode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoEncoderEncodeOptions`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "$0.encode($1, $2)"]
pub fn encode_with_options(&self, frame: &VideoFrame, options: &VideoEncoderEncodeOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "$0.flush()"]
pub fn flush(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoEncoderConfig")]
    #[doc = "The `isConfigSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/isConfigSupported_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`, `VideoEncoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "VideoEncoder.isConfigSupported"]
pub fn is_config_supported(config: &VideoEncoderConfig) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoEncoder/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoder`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoEncoder {
    #[js::import = "$0.reset()"]
pub fn reset(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
