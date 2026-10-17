#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChannelMergerNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelMergerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelMergerNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ChannelMergerNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new ChannelMergerNode(..)` constructor, creating a new instance of `ChannelMergerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelMergerNode/ChannelMergerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelMergerNode`*"]
impl ChannelMergerNode {
    #[js::import = "new ChannelMergerNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<ChannelMergerNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "ChannelMergerOptions",))]
    #[doc = "The `new ChannelMergerNode(..)` constructor, creating a new instance of `ChannelMergerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChannelMergerNode/ChannelMergerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ChannelMergerNode`, `ChannelMergerOptions`*"]
impl ChannelMergerNode {
    #[js::import = "new ChannelMergerNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &ChannelMergerOptions,) -> Result<ChannelMergerNode, JsValue> { panic!("web-sys intrinsic") }
}
