#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DecompressionStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecompressionStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct DecompressionStream(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ReadableStream")]
    #[doc = "Getter for the `readable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream/readable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecompressionStream`, `ReadableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl DecompressionStream {
    #[js::import = "$0.readable"]
pub fn readable(&self) -> ReadableStream { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "WritableStream")]
    #[doc = "Getter for the `writable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream/writable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecompressionStream`, `WritableStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl DecompressionStream {
    #[js::import = "$0.writable"]
pub fn writable(&self) -> WritableStream { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "CompressionFormat")]
    #[doc = "The `new DecompressionStream(..)` constructor, creating a new instance of `DecompressionStream`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream/DecompressionStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CompressionFormat`, `DecompressionStream`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl DecompressionStream {
    #[js::import = "new DecompressionStream($0)"]
pub fn new(format: CompressionFormat) -> Result<DecompressionStream, JsValue> { panic!("web-sys intrinsic") }
}
