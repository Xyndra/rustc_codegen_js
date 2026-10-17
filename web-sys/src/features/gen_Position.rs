#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Position` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Position`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Position(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Coordinates")]
    #[doc = "Getter for the `coords` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/coords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Coordinates`, `Position`*"]
impl Position {
    #[js::import = "$0.coords"]
pub fn coords(&self) -> Coordinates { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Position/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Position`*"]
impl Position {
    #[js::import = "$0.timestamp"]
pub fn timestamp(&self) -> f64 { panic!("web-sys intrinsic") }
}
