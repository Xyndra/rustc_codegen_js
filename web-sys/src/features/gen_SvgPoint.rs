#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPoint` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgPoint(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
impl SvgPoint {
    #[js::import = "$0.x"]
pub fn x(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
impl SvgPoint {
    #[js::import = "$0.x = $1"]
pub fn set_x(&self, value: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
impl SvgPoint {
    #[js::import = "$0.y"]
pub fn y(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`*"]
impl SvgPoint {
    #[js::import = "$0.y = $1"]
pub fn set_y(&self, value: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgMatrix")]
    #[doc = "The `matrixTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPoint/matrixTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgPoint`*"]
impl SvgPoint {
    #[js::import = "$0.matrixTransform($1)"]
pub fn matrix_transform(&self, matrix: &SvgMatrix) -> SvgPoint { panic!("web-sys intrinsic") }
}
