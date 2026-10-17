#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScriptProcessorNode` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ScriptProcessorNode(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `onaudioprocess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
impl ScriptProcessorNode {
    #[js::import = "$0.onaudioprocess"]
pub fn onaudioprocess(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onaudioprocess` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
impl ScriptProcessorNode {
    #[js::import = "$0.onaudioprocess = $1"]
pub fn set_onaudioprocess(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bufferSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/bufferSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScriptProcessorNode`*"]
impl ScriptProcessorNode {
    #[js::import = "$0.bufferSize"]
pub fn buffer_size(&self) -> i32 { panic!("web-sys intrinsic") }
}
