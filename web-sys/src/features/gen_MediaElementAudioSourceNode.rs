#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaElementAudioSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaElementAudioSourceNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaElementAudioSourceNode(pub ::wasm_bindgen::JsValue);
    #[cfg(all(feature = "AudioContext", feature = "MediaElementAudioSourceOptions",))]
    #[doc = "The `new MediaElementAudioSourceNode(..)` constructor, creating a new instance of `MediaElementAudioSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaElementAudioSourceNode/MediaElementAudioSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `MediaElementAudioSourceNode`, `MediaElementAudioSourceOptions`*"]
impl MediaElementAudioSourceNode {
    #[js::import = "new MediaElementAudioSourceNode($0, $1)"]
pub fn new(context: &AudioContext, options: &MediaElementAudioSourceOptions,) -> Result<MediaElementAudioSourceNode, JsValue> { panic!("web-sys intrinsic") }
}
