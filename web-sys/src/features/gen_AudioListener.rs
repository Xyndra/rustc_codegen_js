#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioListener` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AudioListener(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `dopplerFactor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
impl AudioListener {
    #[js::import = "$0.dopplerFactor"]
pub fn doppler_factor(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `dopplerFactor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/dopplerFactor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
impl AudioListener {
    #[js::import = "$0.dopplerFactor = $1"]
pub fn set_doppler_factor(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `speedOfSound` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
impl AudioListener {
    #[js::import = "$0.speedOfSound"]
pub fn speed_of_sound(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `speedOfSound` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/speedOfSound)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
impl AudioListener {
    #[js::import = "$0.speedOfSound = $1"]
pub fn set_speed_of_sound(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setOrientation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setOrientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
impl AudioListener {
    #[js::import = "$0.setOrientation($1, $2, $3, $4, $5, $6)"]
pub fn set_orientation(&self, x: f64, y: f64, z: f64, x_up: f64, y_up: f64, z_up: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setPosition()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
impl AudioListener {
    #[js::import = "$0.setPosition($1, $2, $3)"]
pub fn set_position(&self, x: f64, y: f64, z: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setVelocity()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setVelocity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioListener`*"]
impl AudioListener {
    #[js::import = "$0.setVelocity($1, $2, $3)"]
pub fn set_velocity(&self, x: f64, y: f64, z: f64) { panic!("web-sys intrinsic") }
}
