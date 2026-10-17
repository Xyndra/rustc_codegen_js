#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct GamepadEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Gamepad")]
    #[doc = "Getter for the `gamepad` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/gamepad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadEvent`*"]
impl GamepadEvent {
    #[js::import = "$0.gamepad"]
pub fn gamepad(&self) -> Option<Gamepad> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEvent`*"]
impl GamepadEvent {
    #[js::import = "new GamepadEvent($0)"]
pub fn new(type_: &str) -> Result<GamepadEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "GamepadEventInit")]
    #[doc = "The `new GamepadEvent(..)` constructor, creating a new instance of `GamepadEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadEvent/GamepadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEvent`, `GamepadEventInit`*"]
impl GamepadEvent {
    #[js::import = "new GamepadEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &GamepadEventInit,) -> Result<GamepadEvent, JsValue> { panic!("web-sys intrinsic") }
}
