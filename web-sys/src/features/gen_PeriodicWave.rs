#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PeriodicWave` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWave`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PeriodicWave(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "BaseAudioContext")]
    #[doc = "The `new PeriodicWave(..)` constructor, creating a new instance of `PeriodicWave`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave/PeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`*"]
impl PeriodicWave {
    #[js::import = "new PeriodicWave($0)"]
pub fn new(context: &BaseAudioContext) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "BaseAudioContext", feature = "PeriodicWaveOptions",))]
    #[doc = "The `new PeriodicWave(..)` constructor, creating a new instance of `PeriodicWave`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PeriodicWave/PeriodicWave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseAudioContext`, `PeriodicWave`, `PeriodicWaveOptions`*"]
impl PeriodicWave {
    #[js::import = "new PeriodicWave($0, $1)"]
pub fn new_with_options(context: &BaseAudioContext, options: &PeriodicWaveOptions,) -> Result<PeriodicWave, JsValue> { panic!("web-sys intrinsic") }
}
