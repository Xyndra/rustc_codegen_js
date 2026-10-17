#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioDestinationNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDestinationNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDestinationNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioDestinationNode(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `maxChannelCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioDestinationNode/maxChannelCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioDestinationNode`*"]
impl AudioDestinationNode {
    #[js::import = "$0.maxChannelCount"]
pub fn max_channel_count(&self) -> u32 { panic!("web-sys intrinsic") }
}
