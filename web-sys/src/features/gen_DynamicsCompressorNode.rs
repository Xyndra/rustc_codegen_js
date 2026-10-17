#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DynamicsCompressorNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DynamicsCompressorNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/threshold)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
impl DynamicsCompressorNode {
    #[js::import = "$0.threshold"]
pub fn threshold(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `knee` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/knee)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
impl DynamicsCompressorNode {
    #[js::import = "$0.knee"]
pub fn knee(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `ratio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/ratio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
impl DynamicsCompressorNode {
    #[js::import = "$0.ratio"]
pub fn ratio(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `reduction` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/reduction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DynamicsCompressorNode`*"]
impl DynamicsCompressorNode {
    #[js::import = "$0.reduction"]
pub fn reduction(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `attack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/attack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
impl DynamicsCompressorNode {
    #[js::import = "$0.attack"]
pub fn attack(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `release` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/release)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `DynamicsCompressorNode`*"]
impl DynamicsCompressorNode {
    #[js::import = "$0.release"]
pub fn release(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`*"]
impl DynamicsCompressorNode {
    #[js::import = "new DynamicsCompressorNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<DynamicsCompressorNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "DynamicsCompressorOptions",))]
    #[doc = "The `new DynamicsCompressorNode(..)` constructor, creating a new instance of `DynamicsCompressorNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/DynamicsCompressorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `DynamicsCompressorNode`, `DynamicsCompressorOptions`*"]
impl DynamicsCompressorNode {
    #[js::import = "new DynamicsCompressorNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &DynamicsCompressorOptions,) -> Result<DynamicsCompressorNode, JsValue> { panic!("web-sys intrinsic") }
}
