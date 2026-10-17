#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DelayNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DelayNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DelayNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `delayTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/delayTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DelayNode`*"]
impl DelayNode {
    #[js::import = "$0.delayTime"]
pub fn delay_time(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new DelayNode(..)` constructor, creating a new instance of `DelayNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/DelayNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`*"]
impl DelayNode {
    #[js::import = "new DelayNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<DelayNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "DelayOptions",))]
    #[doc = "The `new DelayNode(..)` constructor, creating a new instance of `DelayNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/DelayNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DelayNode`, `DelayOptions`*"]
impl DelayNode {
    #[js::import = "new DelayNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &DelayOptions,) -> Result<DelayNode, JsValue> { panic!("web-sys intrinsic") }
}
