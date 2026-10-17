#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceRotationRateInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceRotationRateInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
#[js::import = "None"]
pub fn get_alpha(this: &DeviceRotationRateInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
#[js::import = "None = $1"]
pub fn set_alpha(this: &DeviceRotationRateInit, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
#[js::import = "None"]
pub fn get_beta(this: &DeviceRotationRateInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
#[js::import = "None = $1"]
pub fn set_beta(this: &DeviceRotationRateInit, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
#[js::import = "None"]
pub fn get_gamma(this: &DeviceRotationRateInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
#[js::import = "None = $1"]
pub fn set_gamma(this: &DeviceRotationRateInit, val: Option<f64>) { panic!("web-sys intrinsic") }
impl DeviceRotationRateInit {
    #[doc = "Construct a new `DeviceRotationRateInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceRotationRateInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_alpha()` instead."]
#[js::import = "None"]
pub fn alpha(&mut self, val: Option<f64>) -> &mut Self { self.set_alpha(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_beta()` instead."]
#[js::import = "None"]
pub fn beta(&mut self, val: Option<f64>) -> &mut Self { self.set_beta(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_gamma()` instead."]
#[js::import = "None"]
pub fn gamma(&mut self, val: Option<f64>) -> &mut Self { self.set_gamma(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DeviceRotationRateInit {
    fn default() -> Self {
        Self::new()
    }
}
