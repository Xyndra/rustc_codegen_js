#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PeriodicWaveOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PeriodicWaveOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
#[js::import = "None"]
pub fn get_disable_normalization(this: &PeriodicWaveOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
#[js::import = "None = $1"]
pub fn set_disable_normalization(this: &PeriodicWaveOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `imag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
#[js::import = "None"]
pub fn get_imag(this: &PeriodicWaveOptions) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `imag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
#[js::import = "None = $1"]
pub fn set_imag(this: &PeriodicWaveOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `real` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
#[js::import = "None"]
pub fn get_real(this: &PeriodicWaveOptions) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `real` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
#[js::import = "None = $1"]
pub fn set_real(this: &PeriodicWaveOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl PeriodicWaveOptions {
    #[doc = "Construct a new `PeriodicWaveOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_disable_normalization()` instead."]
#[js::import = "None"]
pub fn disable_normalization(&mut self, val: bool) -> &mut Self { self.set_disable_normalization(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_imag()` instead."]
#[js::import = "None"]
pub fn imag(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_imag(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_real()` instead."]
#[js::import = "None"]
pub fn real(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_real(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PeriodicWaveOptions {
    fn default() -> Self {
        Self::new()
    }
}
