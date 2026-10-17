#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChannelSplitterNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelSplitterNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ChannelSplitterNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new ChannelSplitterNode(..)` constructor, creating a new instance of `ChannelSplitterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode/ChannelSplitterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`*"]
impl ChannelSplitterNode {
    #[js::import = "new ChannelSplitterNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<ChannelSplitterNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "ChannelSplitterOptions",))]
    #[doc = "The `new ChannelSplitterNode(..)` constructor, creating a new instance of `ChannelSplitterNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelSplitterNode/ChannelSplitterNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelSplitterNode`, `ChannelSplitterOptions`*"]
impl ChannelSplitterNode {
    #[js::import = "new ChannelSplitterNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &ChannelSplitterOptions,) -> Result<ChannelSplitterNode, JsValue> { panic!("web-sys intrinsic") }
}
