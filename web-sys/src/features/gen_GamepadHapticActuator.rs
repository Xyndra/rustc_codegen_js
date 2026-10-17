#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GamepadHapticActuator` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
#[js::opaque]
#[derive(Copy)]
pub struct GamepadHapticActuator(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "GamepadHapticActuatorType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`, `GamepadHapticActuatorType`*"]
    #[deprecated]
impl GamepadHapticActuator {
    #[js::import = "$0.type"]
pub fn type_(&self) -> GamepadHapticActuatorType { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `effects` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/effects)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GamepadHapticActuator {
    #[js::import = "$0.effects"]
pub fn effects(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GamepadHapticEffectType")]
    #[doc = "The `playEffect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/playEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`, `GamepadHapticEffectType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GamepadHapticActuator {
    #[js::import = "$0.playEffect($1)"]
pub fn play_effect(&self, type_: GamepadHapticEffectType) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(
        feature = "GamepadEffectParameters",
        feature = "GamepadHapticEffectType",
    ))]
    #[doc = "The `playEffect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/playEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadEffectParameters`, `GamepadHapticActuator`, `GamepadHapticEffectType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GamepadHapticActuator {
    #[js::import = "$0.playEffect($1, $2)"]
pub fn play_effect_with_params(&self, type_: GamepadHapticEffectType, params: &GamepadEffectParameters) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `pulse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/pulse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
impl GamepadHapticActuator {
    #[js::import = "$0.pulse($1, $2)"]
pub fn pulse(&self, value: f64, duration: f64) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GamepadHapticActuator/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GamepadHapticActuator`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl GamepadHapticActuator {
    #[js::import = "$0.reset()"]
pub fn reset(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
