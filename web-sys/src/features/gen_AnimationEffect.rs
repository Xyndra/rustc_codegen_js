#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationEffect` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnimationEffect(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ComputedEffectTiming")]
    #[doc = "The `getComputedTiming()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getComputedTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`, `ComputedEffectTiming`*"]
impl AnimationEffect {
    #[js::import = "$0.getComputedTiming()"]
pub fn get_computed_timing(&self) -> ComputedEffectTiming { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EffectTiming")]
    #[doc = "The `getTiming()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/getTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`, `EffectTiming`*"]
impl AnimationEffect {
    #[js::import = "$0.getTiming()"]
pub fn get_timing(&self) -> EffectTiming { panic!("web-sys intrinsic") }
}
    #[doc = "The `updateTiming()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`*"]
impl AnimationEffect {
    #[js::import = "$0.updateTiming()"]
pub fn update_timing(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OptionalEffectTiming")]
    #[doc = "The `updateTiming()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AnimationEffect/updateTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationEffect`, `OptionalEffectTiming`*"]
impl AnimationEffect {
    #[js::import = "$0.updateTiming($1)"]
pub fn update_timing_with_timing(&self, timing: &OptionalEffectTiming) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
