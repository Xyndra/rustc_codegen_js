#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceOrientationEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceOrientationEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &DeviceOrientationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &DeviceOrientationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &DeviceOrientationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &DeviceOrientationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &DeviceOrientationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &DeviceOrientationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `absolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None"]
pub fn get_absolute(this: &DeviceOrientationEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `absolute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_absolute(this: &DeviceOrientationEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None"]
pub fn get_alpha(this: &DeviceOrientationEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_alpha(this: &DeviceOrientationEventInit, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None"]
pub fn get_beta(this: &DeviceOrientationEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `beta` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_beta(this: &DeviceOrientationEventInit, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None"]
pub fn get_gamma(this: &DeviceOrientationEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `gamma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None = $1"]
pub fn set_gamma(this: &DeviceOrientationEventInit, val: Option<f64>) { panic!("web-sys intrinsic") }
impl DeviceOrientationEventInit {
    #[doc = "Construct a new `DeviceOrientationEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceOrientationEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_absolute()` instead."]
#[js::import = "None"]
pub fn absolute(&mut self, val: bool) -> &mut Self { self.set_absolute(val) { panic!("web-sys intrinsic") }
        self
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
impl Default for DeviceOrientationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
