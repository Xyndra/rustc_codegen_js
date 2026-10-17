#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProcessingInstruction` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProcessingInstruction`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ProcessingInstruction(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProcessingInstruction`*"]
impl ProcessingInstruction {
    #[js::import = "$0.target"]
pub fn target(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "StyleSheet")]
    #[doc = "Getter for the `sheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ProcessingInstruction/sheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProcessingInstruction`, `StyleSheet`*"]
impl ProcessingInstruction {
    #[js::import = "$0.sheet"]
pub fn sheet(&self) -> Option<StyleSheet> { panic!("web-sys intrinsic") }
}
