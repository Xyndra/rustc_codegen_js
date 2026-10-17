#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamAudioSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamAudioSourceNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStreamAudioSourceNode(pub ::wasm_bindgen::JsValue);
    #[cfg(all(feature = "AudioContext", feature = "MediaStreamAudioSourceOptions",))]
    #[doc = "The `new MediaStreamAudioSourceNode(..)` constructor, creating a new instance of `MediaStreamAudioSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioSourceNode/MediaStreamAudioSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioSourceNode`, `MediaStreamAudioSourceOptions`*"]
impl MediaStreamAudioSourceNode {
    #[js::import = "new MediaStreamAudioSourceNode($0, $1)"]
pub fn new(context: &AudioContext, options: &MediaStreamAudioSourceOptions,) -> Result<MediaStreamAudioSourceNode, JsValue> { panic!("web-sys intrinsic") }
}
