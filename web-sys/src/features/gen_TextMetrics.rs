#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextMetrics` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TextMetrics(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
impl TextMetrics {
    #[js::import = "$0.width"]
pub fn width(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `actualBoundingBoxLeft` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxLeft)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
impl TextMetrics {
    #[js::import = "$0.actualBoundingBoxLeft"]
pub fn actual_bounding_box_left(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `actualBoundingBoxRight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxRight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
impl TextMetrics {
    #[js::import = "$0.actualBoundingBoxRight"]
pub fn actual_bounding_box_right(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fontBoundingBoxAscent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/fontBoundingBoxAscent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
impl TextMetrics {
    #[js::import = "$0.fontBoundingBoxAscent"]
pub fn font_bounding_box_ascent(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fontBoundingBoxDescent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/fontBoundingBoxDescent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
impl TextMetrics {
    #[js::import = "$0.fontBoundingBoxDescent"]
pub fn font_bounding_box_descent(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `actualBoundingBoxAscent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxAscent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
impl TextMetrics {
    #[js::import = "$0.actualBoundingBoxAscent"]
pub fn actual_bounding_box_ascent(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `actualBoundingBoxDescent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics/actualBoundingBoxDescent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextMetrics`*"]
impl TextMetrics {
    #[js::import = "$0.actualBoundingBoxDescent"]
pub fn actual_bounding_box_descent(&self) -> f64 { panic!("web-sys intrinsic") }
}
