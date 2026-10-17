#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceProximityEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceProximityEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &DeviceProximityEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &DeviceProximityEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &DeviceProximityEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &DeviceProximityEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &DeviceProximityEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &DeviceProximityEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None"]
pub fn get_max(this: &DeviceProximityEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None = $1"]
pub fn set_max(this: &DeviceProximityEventInit, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None"]
pub fn get_min(this: &DeviceProximityEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None = $1"]
pub fn set_min(this: &DeviceProximityEventInit, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None"]
pub fn get_value(this: &DeviceProximityEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
#[js::import = "None = $1"]
pub fn set_value(this: &DeviceProximityEventInit, val: f64) { panic!("web-sys intrinsic") }
impl DeviceProximityEventInit {
    #[doc = "Construct a new `DeviceProximityEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceProximityEventInit`*"]
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
    #[deprecated = "Use `set_max()` instead."]
#[js::import = "None"]
pub fn max(&mut self, val: f64) -> &mut Self { self.set_max(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_min()` instead."]
#[js::import = "None"]
pub fn min(&mut self, val: f64) -> &mut Self { self.set_min(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_value()` instead."]
#[js::import = "None"]
pub fn value(&mut self, val: f64) -> &mut Self { self.set_value(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DeviceProximityEventInit {
    fn default() -> Self {
        Self::new()
    }
}
