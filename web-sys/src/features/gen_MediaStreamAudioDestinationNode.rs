#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamAudioDestinationNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamAudioDestinationNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaStreamAudioDestinationNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaStream")]
    #[doc = "Getter for the `stream` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/stream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioDestinationNode`*"]
impl MediaStreamAudioDestinationNode {
    #[js::import = "$0.stream"]
pub fn stream(&self) -> MediaStream { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioContext")]
    #[doc = "The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new instance of `MediaStreamAudioDestinationNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/MediaStreamAudioDestinationNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `MediaStreamAudioDestinationNode`*"]
impl MediaStreamAudioDestinationNode {
    #[js::import = "new MediaStreamAudioDestinationNode($0)"]
pub fn new(context: &AudioContext) -> Result<MediaStreamAudioDestinationNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "AudioContext", feature = "AudioNodeOptions",))]
    #[doc = "The `new MediaStreamAudioDestinationNode(..)` constructor, creating a new instance of `MediaStreamAudioDestinationNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaStreamAudioDestinationNode/MediaStreamAudioDestinationNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioContext`, `AudioNodeOptions`, `MediaStreamAudioDestinationNode`*"]
impl MediaStreamAudioDestinationNode {
    #[js::import = "new MediaStreamAudioDestinationNode($0, $1)"]
pub fn new_with_options(context: &AudioContext, options: &AudioNodeOptions,) -> Result<MediaStreamAudioDestinationNode, JsValue> { panic!("web-sys intrinsic") }
}
