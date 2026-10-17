#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GainNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GainNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct GainNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `gain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/gain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `GainNode`*"]
impl GainNode {
    #[js::import = "$0.gain"]
pub fn gain(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new GainNode(..)` constructor, creating a new instance of `GainNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`*"]
impl GainNode {
    #[js::import = "new GainNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<GainNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "GainOptions",))]
    #[doc = "The `new GainNode(..)` constructor, creating a new instance of `GainNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/GainNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `GainNode`, `GainOptions`*"]
impl GainNode {
    #[js::import = "new GainNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &GainOptions,) -> Result<GainNode, JsValue> { panic!("web-sys intrinsic") }
}
