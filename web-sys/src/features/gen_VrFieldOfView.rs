#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VrFieldOfView` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
#[js::opaque]
#[derive(Copy)]
pub struct VrFieldOfView(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `upDegrees` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/upDegrees)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
impl VrFieldOfView {
    #[js::import = "$0.upDegrees"]
pub fn up_degrees(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rightDegrees` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/rightDegrees)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
impl VrFieldOfView {
    #[js::import = "$0.rightDegrees"]
pub fn right_degrees(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `downDegrees` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/downDegrees)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
impl VrFieldOfView {
    #[js::import = "$0.downDegrees"]
pub fn down_degrees(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `leftDegrees` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VRFieldOfView/leftDegrees)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VrFieldOfView`*"]
impl VrFieldOfView {
    #[js::import = "$0.leftDegrees"]
pub fn left_degrees(&self) -> f64 { panic!("web-sys intrinsic") }
}
