#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioScheduledSourceNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioScheduledSourceNode(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
impl AudioScheduledSourceNode {
    #[js::import = "$0.onended"]
pub fn onended(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
impl AudioScheduledSourceNode {
    #[js::import = "$0.onended = $1"]
pub fn set_onended(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
impl AudioScheduledSourceNode {
    #[js::import = "$0.start()"]
pub fn start(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
impl AudioScheduledSourceNode {
    #[js::import = "$0.start($1)"]
pub fn start_with_when(&self, when: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
impl AudioScheduledSourceNode {
    #[js::import = "$0.stop()"]
pub fn stop(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioScheduledSourceNode`*"]
impl AudioScheduledSourceNode {
    #[js::import = "$0.stop($1)"]
pub fn stop_with_when(&self, when: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
