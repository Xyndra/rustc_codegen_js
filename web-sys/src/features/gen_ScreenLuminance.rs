#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScreenLuminance` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenLuminance`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ScreenLuminance(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `min` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/min)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenLuminance`*"]
impl ScreenLuminance {
    #[js::import = "$0.min"]
pub fn min(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `max` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/max)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenLuminance`*"]
impl ScreenLuminance {
    #[js::import = "$0.max"]
pub fn max(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `maxAverage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ScreenLuminance/maxAverage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScreenLuminance`*"]
impl ScreenLuminance {
    #[js::import = "$0.maxAverage"]
pub fn max_average(&self) -> f64 { panic!("web-sys intrinsic") }
}
