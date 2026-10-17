#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomPointReadOnly` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomPointReadOnly(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "$0.x"]
pub fn x(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "$0.y"]
pub fn y(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `z` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/z)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "$0.z"]
pub fn z(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `w` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/w)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "$0.w"]
pub fn w(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "new DOMPointReadOnly()"]
pub fn new() -> Result<DomPointReadOnly, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "new DOMPointReadOnly($0)"]
pub fn new_with_x(x: f64) -> Result<DomPointReadOnly, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "new DOMPointReadOnly($0, $1)"]
pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomPointReadOnly, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "new DOMPointReadOnly($0, $1, $2)"]
pub fn new_with_x_and_y_and_z(x: f64, y: f64, z: f64) -> Result<DomPointReadOnly, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomPointReadOnly(..)` constructor, creating a new instance of `DomPointReadOnly`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/DOMPointReadOnly)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "new DOMPointReadOnly($0, $1, $2, $3)"]
pub fn new_with_x_and_y_and_z_and_w(x: f64, y: f64, z: f64, w: f64,) -> Result<DomPointReadOnly, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `fromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "DOMPointReadOnly.fromPoint"]
pub fn from_point() -> DomPointReadOnly { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `fromPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/fromPoint_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "DOMPointReadOnly.fromPoint"]
pub fn from_point_with_other(other: &DomPointInit) -> DomPointReadOnly { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMPointReadOnly/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`*"]
impl DomPointReadOnly {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
