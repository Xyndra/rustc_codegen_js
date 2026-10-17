#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EncodedVideoChunkMetadata` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct EncodedVideoChunkMetadata(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `alphaSideData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_alpha_side_data(this: &EncodedVideoChunkMetadata) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `alphaSideData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_alpha_side_data(this: &EncodedVideoChunkMetadata, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "Get the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_decoder_config(this: &EncodedVideoChunkMetadata) -> Option<VideoDecoderConfig> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[doc = "Change the `decoderConfig` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `VideoDecoderConfig`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_decoder_config(this: &EncodedVideoChunkMetadata, val: &VideoDecoderConfig) { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvcOutputMetadata")]
    #[doc = "Get the `svc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn get_svc(this: &EncodedVideoChunkMetadata) -> Option<SvcOutputMetadata> { panic!("web-sys intrinsic") }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvcOutputMetadata")]
    #[doc = "Change the `svc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`, `SvcOutputMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None = $1"]
pub fn set_svc(this: &EncodedVideoChunkMetadata, val: &SvcOutputMetadata) { panic!("web-sys intrinsic") }
#[cfg(web_sys_unstable_apis)]
impl EncodedVideoChunkMetadata {
    #[doc = "Construct a new `EncodedVideoChunkMetadata`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EncodedVideoChunkMetadata`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[deprecated = "Use `set_alpha_side_data()` instead."]
#[js::import = "None"]
pub fn alpha_side_data(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_alpha_side_data(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoDecoderConfig")]
    #[deprecated = "Use `set_decoder_config()` instead."]
#[js::import = "None"]
pub fn decoder_config(&mut self, val: &VideoDecoderConfig) -> &mut Self { self.set_decoder_config(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvcOutputMetadata")]
    #[deprecated = "Use `set_svc()` instead."]
#[js::import = "None"]
pub fn svc(&mut self, val: &SvcOutputMetadata) -> &mut Self { self.set_svc(val) { panic!("web-sys intrinsic") }
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for EncodedVideoChunkMetadata {
    fn default() -> Self {
        Self::new()
    }
}
