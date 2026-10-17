#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebKitCssMatrix` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebKitCssMatrix(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "new WebKitCSSMatrix()"]
pub fn new() -> Result<WebKitCssMatrix, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "new WebKitCSSMatrix($0)"]
pub fn new_with_transform_list(transform_list: &str) -> Result<WebKitCssMatrix, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new WebKitCssMatrix(..)` constructor, creating a new instance of `WebKitCssMatrix`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/WebKitCSSMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "new WebKitCSSMatrix($0)"]
pub fn new_with_other(other: &WebKitCssMatrix) -> Result<WebKitCssMatrix, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `inverse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/inverse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.inverse()"]
pub fn inverse(&self) -> Result<WebKitCssMatrix, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `multiply()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/multiply)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.multiply($1)"]
pub fn multiply(&self, other: &WebKitCssMatrix) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.rotate()"]
pub fn rotate(&self) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.rotate($1)"]
pub fn rotate_with_rot_x(&self, rot_x: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.rotate($1, $2)"]
pub fn rotate_with_rot_x_and_rot_y(&self, rot_x: f64, rot_y: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.rotate($1, $2, $3)"]
pub fn rotate_with_rot_x_and_rot_y_and_rot_z(&self, rot_x: f64, rot_y: f64, rot_z: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.rotateAxisAngle()"]
pub fn rotate_axis_angle(&self) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.rotateAxisAngle($1)"]
pub fn rotate_axis_angle_with_x(&self, x: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.rotateAxisAngle($1, $2)"]
pub fn rotate_axis_angle_with_x_and_y(&self, x: f64, y: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.rotateAxisAngle($1, $2, $3)"]
pub fn rotate_axis_angle_with_x_and_y_and_z(&self, x: f64, y: f64, z: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotateAxisAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/rotateAxisAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.rotateAxisAngle($1, $2, $3, $4)"]
pub fn rotate_axis_angle_with_x_and_y_and_z_and_angle(&self, x: f64, y: f64, z: f64, angle: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.scale()"]
pub fn scale(&self) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.scale($1)"]
pub fn scale_with_scale_x(&self, scale_x: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.scale($1, $2)"]
pub fn scale_with_scale_x_and_scale_y(&self, scale_x: f64, scale_y: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.scale($1, $2, $3)"]
pub fn scale_with_scale_x_and_scale_y_and_scale_z(&self, scale_x: f64, scale_y: f64, scale_z: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `setMatrixValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/setMatrixValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.setMatrixValue($1)"]
pub fn set_matrix_value(&self, transform_list: &str) -> Result<WebKitCssMatrix, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `skewX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.skewX()"]
pub fn skew_x(&self) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `skewX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.skewX($1)"]
pub fn skew_x_with_sx(&self, sx: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `skewY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.skewY()"]
pub fn skew_y(&self) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `skewY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/skewY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.skewY($1)"]
pub fn skew_y_with_sy(&self, sy: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.translate()"]
pub fn translate(&self) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.translate($1)"]
pub fn translate_with_tx(&self, tx: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.translate($1, $2)"]
pub fn translate_with_tx_and_ty(&self, tx: f64, ty: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebKitCSSMatrix/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebKitCssMatrix`*"]
impl WebKitCssMatrix {
    #[js::import = "$0.translate($1, $2, $3)"]
pub fn translate_with_tx_and_ty_and_tz(&self, tx: f64, ty: f64, tz: f64) -> WebKitCssMatrix { panic!("web-sys intrinsic") }
}
