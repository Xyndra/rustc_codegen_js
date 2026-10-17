#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstantSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConstantSourceNode(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AudioParam")]
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioParam`, `ConstantSourceNode`*"]
impl ConstantSourceNode {
    #[js::import = "$0.offset"]
pub fn offset(&self) -> AudioParam { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
impl ConstantSourceNode {
    #[js::import = "$0.onended"]
pub fn onended(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
impl ConstantSourceNode {
    #[js::import = "$0.onended = $1"]
pub fn set_onended(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`*"]
impl ConstantSourceNode {
    #[js::import = "new ConstantSourceNode($0)"]
pub fn new(context: &BaseAudioContext) -> Result<ConstantSourceNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "ConstantSourceOptions",))]
    #[doc = "The `new ConstantSourceNode(..)` constructor, creating a new instance of `ConstantSourceNode`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/ConstantSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `ConstantSourceNode`, `ConstantSourceOptions`*"]
impl ConstantSourceNode {
    #[js::import = "new ConstantSourceNode($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &ConstantSourceOptions,) -> Result<ConstantSourceNode, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
impl ConstantSourceNode {
    #[js::import = "$0.start()"]
pub fn start(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
impl ConstantSourceNode {
    #[js::import = "$0.start($1)"]
pub fn start_with_when(&self, when: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
impl ConstantSourceNode {
    #[js::import = "$0.stop()"]
pub fn stop(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceNode`*"]
impl ConstantSourceNode {
    #[js::import = "$0.stop($1)"]
pub fn stop_with_when(&self, when: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
