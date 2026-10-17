#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DeviceMotionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DeviceMotionEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &DeviceMotionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &DeviceMotionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &DeviceMotionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &DeviceMotionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &DeviceMotionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &DeviceMotionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Get the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
#[js::import = "None"]
pub fn get_acceleration(this: &DeviceMotionEventInit) -> Option<DeviceAccelerationInit> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Change the `acceleration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_acceleration(this: &DeviceMotionEventInit, val: &DeviceAccelerationInit) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Get the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
#[js::import = "None"]
pub fn get_acceleration_including_gravity(this: &DeviceMotionEventInit,) -> Option<DeviceAccelerationInit> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[doc = "Change the `accelerationIncludingGravity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceAccelerationInit`, `DeviceMotionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_acceleration_including_gravity(this: &DeviceMotionEventInit, val: &DeviceAccelerationInit,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `interval` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
#[js::import = "None"]
pub fn get_interval(this: &DeviceMotionEventInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `interval` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_interval(this: &DeviceMotionEventInit, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DeviceRotationRateInit")]
    #[doc = "Get the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
#[js::import = "None"]
pub fn get_rotation_rate(this: &DeviceMotionEventInit) -> Option<DeviceRotationRateInit> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DeviceRotationRateInit")]
    #[doc = "Change the `rotationRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`, `DeviceRotationRateInit`*"]
#[js::import = "None = $1"]
pub fn set_rotation_rate(this: &DeviceMotionEventInit, val: &DeviceRotationRateInit) { panic!("web-sys intrinsic") }
impl DeviceMotionEventInit {
    #[doc = "Construct a new `DeviceMotionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DeviceMotionEventInit`*"]
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
    #[cfg(feature = "DeviceAccelerationInit")]
    #[deprecated = "Use `set_acceleration()` instead."]
#[js::import = "None"]
pub fn acceleration(&mut self, val: &DeviceAccelerationInit) -> &mut Self { self.set_acceleration(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DeviceAccelerationInit")]
    #[deprecated = "Use `set_acceleration_including_gravity()` instead."]
#[js::import = "None"]
pub fn acceleration_including_gravity(&mut self, val: &DeviceAccelerationInit) -> &mut Self { self.set_acceleration_including_gravity(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_interval()` instead."]
#[js::import = "None"]
pub fn interval(&mut self, val: Option<f64>) -> &mut Self { self.set_interval(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DeviceRotationRateInit")]
    #[deprecated = "Use `set_rotation_rate()` instead."]
#[js::import = "None"]
pub fn rotation_rate(&mut self, val: &DeviceRotationRateInit) -> &mut Self { self.set_rotation_rate(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DeviceMotionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
