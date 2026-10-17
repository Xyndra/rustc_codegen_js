#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRect` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomRect(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "$0.x"]
pub fn x(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "$0.x = $1"]
pub fn set_x(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "$0.y"]
pub fn y(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "$0.y = $1"]
pub fn set_y(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "$0.width"]
pub fn width(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "$0.width = $1"]
pub fn set_width(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "$0.height"]
pub fn height(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "$0.height = $1"]
pub fn set_height(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "new DOMRect()"]
pub fn new() -> Result<DomRect, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "new DOMRect($0)"]
pub fn new_with_x(x: f64) -> Result<DomRect, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "new DOMRect($0, $1)"]
pub fn new_with_x_and_y(x: f64, y: f64) -> Result<DomRect, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "new DOMRect($0, $1, $2)"]
pub fn new_with_x_and_y_and_width(x: f64, y: f64, width: f64) -> Result<DomRect, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomRect(..)` constructor, creating a new instance of `DomRect`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/DOMRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`*"]
impl DomRect {
    #[js::import = "new DOMRect($0, $1, $2, $3)"]
pub fn new_with_x_and_y_and_width_and_height(x: f64, y: f64, width: f64, height: f64,) -> Result<DomRect, JsValue> { panic!("web-sys intrinsic") }
}
