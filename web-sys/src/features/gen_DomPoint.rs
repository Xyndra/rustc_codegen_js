#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomPoint` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomPoint(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "$0.x"]
pub fn x(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "$0.x = $1"]
pub fn set_x(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "$0.y"]
pub fn y(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "$0.y = $1"]
pub fn set_y(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "$0.z"]
pub fn z(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "$0.z = $1"]
pub fn set_z(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `w` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "$0.w"]
pub fn w(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `w` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/w)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "$0.w = $1"]
pub fn set_w(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "new DOMPoint()"]
pub fn new() -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "new DOMPoint($0)"]
pub fn new_with_x(x: f64) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "new DOMPoint($0, $1)"]
pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "new DOMPoint($0, $1, $2)"]
pub fn new_with_x_and_y_and_z(x: f64, y: f64, z: f64) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPoint(..)` constructor, creating a new instance of `DomPoint`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/DOMPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "new DOMPoint($0, $1, $2, $3)"]
pub fn new_with_x_and_y_and_z_and_w(x: f64, y: f64, z: f64, w: f64,) -> Result<DomPoint, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `fromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`*"]
impl DomPoint {
    #[js::import = "DOMPoint.fromPoint"]
pub fn from_point() -> DomPoint { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `fromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPoint/fromPoint_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomPointInit`*"]
impl DomPoint {
    #[js::import = "DOMPoint.fromPoint"]
pub fn from_point_with_other(other: &DomPointInit) -> DomPoint { panic!("web-sys intrinsic") }
}
