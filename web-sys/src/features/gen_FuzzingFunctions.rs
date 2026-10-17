#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FuzzingFunctions` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FuzzingFunctions(pub ::wasm_bindgen::JsValue);
    #[doc = "The `cycleCollect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/cycleCollect_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
impl FuzzingFunctions {
    #[js::import = "FuzzingFunctions.cycleCollect"]
pub fn cycle_collect() { panic!("web-sys intrinsic") }
}
    #[doc = "The `enableAccessibility()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/enableAccessibility_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
impl FuzzingFunctions {
    #[js::import = "FuzzingFunctions.enableAccessibility"]
pub fn enable_accessibility() -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `garbageCollect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FuzzingFunctions/garbageCollect_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FuzzingFunctions`*"]
impl FuzzingFunctions {
    #[js::import = "FuzzingFunctions.garbageCollect"]
pub fn garbage_collect() { panic!("web-sys intrinsic") }
}
