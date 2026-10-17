#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OscillatorNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OscillatorNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "OscillatorType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`, `OscillatorType`*"]
impl OscillatorNode {
    #[js::import = "$0.type"]
pub fn type_(&self) -> OscillatorType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OscillatorType")]
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`, `OscillatorType`*"]
impl OscillatorNode {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: OscillatorType) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `frequency` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/frequency)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `OscillatorNode`*"]
impl OscillatorNode {
    #[js::import = "$0.frequency"]
pub fn frequency(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `detune` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/detune)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `OscillatorNode`*"]
impl OscillatorNode {
    #[js::import = "$0.detune"]
pub fn detune(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
impl OscillatorNode {
    #[js::import = "$0.onended"]
pub fn onended(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
impl OscillatorNode {
    #[js::import = "$0.onended = $1"]
pub fn set_onended(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new OscillatorNode(..)` constructor, creating a new instance of `OscillatorNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/OscillatorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`*"]
impl OscillatorNode {
    #[js::import = "new OscillatorNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<OscillatorNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "OscillatorOptions",))]
    #[doc = "The `new OscillatorNode(..)` constructor, creating a new instance of `OscillatorNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/OscillatorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `OscillatorNode`, `OscillatorOptions`*"]
impl OscillatorNode {
    #[js::import = "new OscillatorNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &OscillatorOptions,) -> Result<OscillatorNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PeriodicWave")]
    #[doc = "The `setPeriodicWave()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/setPeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`, `PeriodicWave`*"]
impl OscillatorNode {
    #[js::import = "$0.setPeriodicWave($1)"]
pub fn set_periodic_wave(&self, periodic_wave: &PeriodicWave) { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
impl OscillatorNode {
    #[js::import = "$0.start()"]
pub fn start(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
impl OscillatorNode {
    #[js::import = "$0.start($1)"]
pub fn start_with_when(&self, when: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
impl OscillatorNode {
    #[js::import = "$0.stop()"]
pub fn stop(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OscillatorNode`*"]
impl OscillatorNode {
    #[js::import = "$0.stop($1)"]
pub fn stop_with_when(&self, when: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
