#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaCapabilities` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaCapabilities(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaDecodingConfiguration")]
    #[doc = "The `decodingInfo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/decodingInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaDecodingConfiguration`*"]
impl MediaCapabilities {
    #[js::import = "$0.decodingInfo($1)"]
pub fn decoding_info(&self, configuration: &MediaDecodingConfiguration) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaEncodingConfiguration")]
    #[doc = "The `encodingInfo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaCapabilities/encodingInfo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaCapabilities`, `MediaEncodingConfiguration`*"]
impl MediaCapabilities {
    #[js::import = "$0.encodingInfo($1)"]
pub fn encoding_info(&self, configuration: &MediaEncodingConfiguration) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
