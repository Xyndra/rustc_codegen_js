#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Coordinates` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Coordinates(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `latitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/latitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
impl Coordinates {
    #[js::import = "$0.latitude"]
pub fn latitude(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `longitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/longitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
impl Coordinates {
    #[js::import = "$0.longitude"]
pub fn longitude(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `altitude` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitude)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
impl Coordinates {
    #[js::import = "$0.altitude"]
pub fn altitude(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `accuracy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/accuracy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
impl Coordinates {
    #[js::import = "$0.accuracy"]
pub fn accuracy(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `altitudeAccuracy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/altitudeAccuracy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
impl Coordinates {
    #[js::import = "$0.altitudeAccuracy"]
pub fn altitude_accuracy(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `heading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/heading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
impl Coordinates {
    #[js::import = "$0.heading"]
pub fn heading(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `speed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Coordinates/speed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`*"]
impl Coordinates {
    #[js::import = "$0.speed"]
pub fn speed(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
