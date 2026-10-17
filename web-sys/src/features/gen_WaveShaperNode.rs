#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WaveShaperNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WaveShaperNode(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `curve` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
impl WaveShaperNode {
    #[js::import = "$0.curve"]
pub fn curve(&self) -> Option<::alloc::vec::Vec<f32>> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `curve` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
    #[deprecated]
impl WaveShaperNode {
    #[js::import = "$0.curve = $1"]
pub fn set_curve(&self, value: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `curve` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
impl WaveShaperNode {
    #[js::import = "$0.curve = $1"]
pub fn set_curve_opt_f32_slice(&self, value: Option<&mut [f32]>) { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `curve` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WaveShaperNode`*"]
impl WaveShaperNode {
    #[js::import = "$0.curve = $1"]
pub fn set_curve_opt_f32_array(&self, value: Option<&::js_sys::Float32Array>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OverSampleType")]
    #[doc = "Getter for the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperNode`*"]
impl WaveShaperNode {
    #[js::import = "$0.oversample"]
pub fn oversample(&self) -> OverSampleType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OverSampleType")]
    #[doc = "Setter for the `oversample` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OverSampleType`, `WaveShaperNode`*"]
impl WaveShaperNode {
    #[js::import = "$0.oversample = $1"]
pub fn set_oversample(&self, value: OverSampleType) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new WaveShaperNode(..)` constructor, creating a new instance of `WaveShaperNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/WaveShaperNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`*"]
impl WaveShaperNode {
    #[js::import = "new WaveShaperNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<WaveShaperNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "WaveShaperOptions",))]
    #[doc = "The `new WaveShaperNode(..)` constructor, creating a new instance of `WaveShaperNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/WaveShaperNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `WaveShaperNode`, `WaveShaperOptions`*"]
impl WaveShaperNode {
    #[js::import = "new WaveShaperNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &WaveShaperOptions,) -> Result<WaveShaperNode, JsValue> { panic!("web-sys intrinsic") }
}
