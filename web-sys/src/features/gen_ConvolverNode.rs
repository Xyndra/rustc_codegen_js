#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConvolverNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConvolverNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Getter for the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*"]
impl ConvolverNode {
    #[js::import = "$0.buffer"]
pub fn buffer(&self) -> Option<AudioBuffer> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioBuffer")]
    #[doc = "Setter for the `buffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioBuffer`, `ConvolverNode`*"]
impl ConvolverNode {
    #[js::import = "$0.buffer = $1"]
pub fn set_buffer(&self, value: Option<&AudioBuffer>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `normalize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverNode`*"]
impl ConvolverNode {
    #[js::import = "$0.normalize"]
pub fn normalize(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `normalize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvolverNode`*"]
impl ConvolverNode {
    #[js::import = "$0.normalize = $1"]
pub fn set_normalize(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`*"]
impl ConvolverNode {
    #[js::import = "new ConvolverNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<ConvolverNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "ConvolverOptions",))]
    #[doc = "The `new ConvolverNode(..)` constructor, creating a new instance of `ConvolverNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/ConvolverNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConvolverNode`, `ConvolverOptions`*"]
impl ConvolverNode {
    #[js::import = "new ConvolverNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &ConvolverOptions,) -> Result<ConvolverNode, JsValue> { panic!("web-sys intrinsic") }
}
