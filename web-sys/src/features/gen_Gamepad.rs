#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Gamepad` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Gamepad(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `id` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
impl Gamepad {
    #[js::import = "$0.id"]
pub fn id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `index` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
impl Gamepad {
    #[js::import = "$0.index"]
pub fn index(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `connected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
impl Gamepad {
    #[js::import = "$0.connected"]
pub fn connected(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
impl Gamepad {
    #[js::import = "$0.timestamp"]
pub fn timestamp(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "GamepadMappingType")]
    #[doc = "Getter for the `mapping` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/mapping)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadMappingType`*"]
impl Gamepad {
    #[js::import = "$0.mapping"]
pub fn mapping(&self) -> GamepadMappingType { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `axes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/axes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
impl Gamepad {
    #[js::import = "$0.axes"]
pub fn axes(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `buttons` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/buttons)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
impl Gamepad {
    #[js::import = "$0.buttons"]
pub fn buttons(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `displayId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/displayId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    #[deprecated]
impl Gamepad {
    #[js::import = "$0.displayId"]
pub fn display_id(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "GamepadHand")]
    #[doc = "Getter for the `hand` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadHand`*"]
impl Gamepad {
    #[js::import = "$0.hand"]
pub fn hand(&self) -> GamepadHand { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `hapticActuators` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/hapticActuators)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
impl Gamepad {
    #[js::import = "$0.hapticActuators"]
pub fn haptic_actuators(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "GamepadPose")]
    #[doc = "Getter for the `pose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/pose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadPose`*"]
impl Gamepad {
    #[js::import = "$0.pose"]
pub fn pose(&self) -> Option<GamepadPose> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GamepadHapticActuator")]
    #[doc = "Getter for the `vibrationActuator` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/vibrationActuator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`, `GamepadHapticActuator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Gamepad {
    #[js::import = "$0.vibrationActuator"]
pub fn vibration_actuator(&self) -> GamepadHapticActuator { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `touchEvents` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/touchEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Gamepad`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Gamepad {
    #[js::import = "$0.touchEvents"]
pub fn touch_events(&self) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
}
