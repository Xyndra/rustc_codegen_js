#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceAccelerationInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceAccelerationInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
#[js::import = "None"]
pub fn get_x(this: &DeviceAccelerationInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
#[js::import = "None = $1"]
pub fn set_x(this: &DeviceAccelerationInit, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
#[js::import = "None"]
pub fn get_y(this: &DeviceAccelerationInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
#[js::import = "None = $1"]
pub fn set_y(this: &DeviceAccelerationInit, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
#[js::import = "None"]
pub fn get_z(this: &DeviceAccelerationInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
#[js::import = "None = $1"]
pub fn set_z(this: &DeviceAccelerationInit, val: Option<f64>) { panic!("web-sys intrinsic") }
impl DeviceAccelerationInit {
    #[doc = "Construct a new `DeviceAccelerationInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_x()` instead."]
#[js::import = "None"]
pub fn x(&mut self, val: Option<f64>) -> &mut Self { self.set_x(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_y()` instead."]
#[js::import = "None"]
pub fn y(&mut self, val: Option<f64>) -> &mut Self { self.set_y(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_z()` instead."]
#[js::import = "None"]
pub fn z(&mut self, val: Option<f64>) -> &mut Self { self.set_z(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DeviceAccelerationInit {
    fn default() -> Self {
        Self::new()
    }
}
