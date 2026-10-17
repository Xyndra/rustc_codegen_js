#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasGradient` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CanvasGradient(pub ::wasm_bindgen::JsValue);
    #[doc = "The `addColorStop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasGradient/addColorStop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`*"]
impl CanvasGradient {
    #[js::import = "$0.addColorStop($1, $2)"]
pub fn add_color_stop(&self, offset: f32, color: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
