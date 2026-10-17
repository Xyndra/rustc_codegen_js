#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedAudioChunkMetadata` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct EncodedAudioChunkMetadata(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    #[doc = "Get the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`, `EncodedAudioChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_decoder_config(this: &EncodedAudioChunkMetadata) -> Option<AudioDecoderConfig> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    #[doc = "Change the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDecoderConfig`, `EncodedAudioChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_decoder_config(this: &EncodedAudioChunkMetadata, val: &AudioDecoderConfig) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl EncodedAudioChunkMetadata {
    #[doc = "Construct a new `EncodedAudioChunkMetadata`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedAudioChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioDecoderConfig")]
    #[deprecated = "Use `set_decoder_config()` instead."]
#[js::import = "None"]
pub fn decoder_config(&mut self, val: &AudioDecoderConfig) -> &mut Self { self.set_decoder_config(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for EncodedAudioChunkMetadata {
    fn default() -> Self {
        Self::new()
    }
}
