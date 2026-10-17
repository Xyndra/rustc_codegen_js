#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StereoPannerNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StereoPannerNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StereoPannerNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `pan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/pan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `StereoPannerNode`*"]
impl StereoPannerNode {
    #[js::import = "$0.pan"]
pub fn pan(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new StereoPannerNode(..)` constructor, creating a new instance of `StereoPannerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/StereoPannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`*"]
impl StereoPannerNode {
    #[js::import = "new StereoPannerNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<StereoPannerNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "StereoPannerOptions",))]
    #[doc = "The `new StereoPannerNode(..)` constructor, creating a new instance of `StereoPannerNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/StereoPannerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `StereoPannerNode`, `StereoPannerOptions`*"]
impl StereoPannerNode {
    #[js::import = "new StereoPannerNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &StereoPannerOptions,) -> Result<StereoPannerNode, JsValue> { panic!("web-sys intrinsic") }
}
