#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PeriodicWaveConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PeriodicWaveConstraints(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
#[js::import = "None"]
pub fn get_disable_normalization(this: &PeriodicWaveConstraints) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
#[js::import = "None = $1"]
pub fn set_disable_normalization(this: &PeriodicWaveConstraints, val: bool) { panic!("web-sys intrinsic") }
impl PeriodicWaveConstraints {
    #[doc = "Construct a new `PeriodicWaveConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveConstraints`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_disable_normalization()` instead."]
#[js::import = "None"]
pub fn disable_normalization(&mut self, val: bool) -> &mut Self { self.set_disable_normalization(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PeriodicWaveConstraints {
    fn default() -> Self {
        Self::new()
    }
}
