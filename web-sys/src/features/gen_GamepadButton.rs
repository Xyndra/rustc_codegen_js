#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadButton` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButton`*"]
#[js::opaque]
#[derive(Copy)]
pub struct GamepadButton(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `pressed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/pressed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButton`*"]
impl GamepadButton {
    #[js::import = "$0.pressed"]
pub fn pressed(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `touched` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/touched)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButton`*"]
impl GamepadButton {
    #[js::import = "$0.touched"]
pub fn touched(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `value` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadButton/value)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadButton`*"]
impl GamepadButton {
    #[js::import = "$0.value"]
pub fn value(&self) -> f64 { panic!("web-sys intrinsic") }
}
