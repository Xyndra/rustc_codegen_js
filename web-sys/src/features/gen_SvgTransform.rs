#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgTransform` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgTransform(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
impl SvgTransform {
    #[js::import = "$0.type"]
pub fn type_(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgMatrix")]
    #[doc = "Getter for the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/matrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*"]
impl SvgTransform {
    #[js::import = "$0.matrix"]
pub fn matrix(&self) -> SvgMatrix { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `angle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/angle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
impl SvgTransform {
    #[js::import = "$0.angle"]
pub fn angle(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgMatrix")]
    #[doc = "The `setMatrix()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`*"]
impl SvgTransform {
    #[js::import = "$0.setMatrix($1)"]
pub fn set_matrix(&self, matrix: &SvgMatrix) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setRotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setRotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
impl SvgTransform {
    #[js::import = "$0.setRotate($1, $2, $3)"]
pub fn set_rotate(&self, angle: f32, cx: f32, cy: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setScale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setScale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
impl SvgTransform {
    #[js::import = "$0.setScale($1, $2)"]
pub fn set_scale(&self, sx: f32, sy: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setSkewX()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
impl SvgTransform {
    #[js::import = "$0.setSkewX($1)"]
pub fn set_skew_x(&self, angle: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setSkewY()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setSkewY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
impl SvgTransform {
    #[js::import = "$0.setSkewY($1)"]
pub fn set_skew_y(&self, angle: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTranslate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransform/setTranslate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
impl SvgTransform {
    #[js::import = "$0.setTranslate($1, $2)"]
pub fn set_translate(&self, tx: f32, ty: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl SvgTransform {
    #[doc = "The `SVGTransform.SVG_TRANSFORM_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_MATRIX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_MATRIX: u16 = 1u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_TRANSLATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_TRANSLATE: u16 = 2u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_SCALE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_SCALE: u16 = 3u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_ROTATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_ROTATE: u16 = 4u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_SKEWX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_SKEWX: u16 = 5u64 as u16;
    #[doc = "The `SVGTransform.SVG_TRANSFORM_SKEWY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`*"]
    pub const SVG_TRANSFORM_SKEWY: u16 = 6u64 as u16;
}
