#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Path2d` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Path2d(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new Path2d(..)` constructor, creating a new instance of `Path2d`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "new Path2D()"]
pub fn new() -> Result<Path2d, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Path2d(..)` constructor, creating a new instance of `Path2d`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "new Path2D($0)"]
pub fn new_with_other(other: &Path2d) -> Result<Path2d, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Path2d(..)` constructor, creating a new instance of `Path2d`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/Path2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "new Path2D($0)"]
pub fn new_with_path_string(path_string: &str) -> Result<Path2d, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `addPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.addPath($1)"]
pub fn add_path(&self, path: &Path2d) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgMatrix")]
    #[doc = "The `addPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/addPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`, `SvgMatrix`*"]
impl Path2d {
    #[js::import = "$0.addPath($1, $2)"]
pub fn add_path_with_transformation(&self, path: &Path2d, transformation: &SvgMatrix) { panic!("web-sys intrinsic") }
}
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.arc($1, $2, $3, $4, $5)"]
pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.arc($1, $2, $3, $4, $5, $6)"]
pub fn arc_with_anticlockwise(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anticlockwise: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `arcTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/arcTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.arcTo($1, $2, $3, $4, $5)"]
pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `bezierCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/bezierCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.bezierCurveTo($1, $2, $3, $4, $5, $6)"]
pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `closePath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/closePath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.closePath()"]
pub fn close_path(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.ellipse($1, $2, $3, $4, $5, $6, $7)"]
pub fn ellipse(&self, x: f64, y: f64, radius_x: f64, radius_y: f64, rotation: f64, start_angle: f64, end_angle: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.ellipse($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn ellipse_with_anticlockwise(&self, x: f64, y: f64, radius_x: f64, radius_y: f64, rotation: f64, start_angle: f64, end_angle: f64, anticlockwise: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `lineTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/lineTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.lineTo($1, $2)"]
pub fn line_to(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `moveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/moveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.moveTo($1, $2)"]
pub fn move_to(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `quadraticCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/quadraticCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.quadraticCurveTo($1, $2, $3, $4)"]
pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `rect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/rect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.rect($1, $2, $3, $4)"]
pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4)"]
pub fn round_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4, $5)"]
pub fn round_rect_with_f64(&self, x: f64, y: f64, w: f64, h: f64, radii: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `Path2d`*"]
impl Path2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4, $5)"]
pub fn round_rect_with_dom_point_init(&self, x: f64, y: f64, w: f64, h: f64, radii: &DomPointInit) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4, $5)"]
pub fn round_rect_with_f64_sequence(&self, x: f64, y: f64, w: f64, h: f64, radii: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Path2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Path2d`*"]
impl Path2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4, $5)"]
pub fn round_rect_with_dom_point_init_sequence(&self, x: f64, y: f64, w: f64, h: f64, radii: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
