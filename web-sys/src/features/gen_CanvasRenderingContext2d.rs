#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasRenderingContext2d` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CanvasRenderingContext2d(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.canvas"]
pub fn canvas(&self) -> Option<HtmlCanvasElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `globalAlpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.globalAlpha"]
pub fn global_alpha(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `globalAlpha` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalAlpha)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.globalAlpha = $1"]
pub fn set_global_alpha(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `globalCompositeOperation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.globalCompositeOperation"]
pub fn global_composite_operation(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `globalCompositeOperation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/globalCompositeOperation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.globalCompositeOperation = $1"]
pub fn set_global_composite_operation(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.strokeStyle"]
pub fn stroke_style(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[deprecated]
impl CanvasRenderingContext2d {
    #[js::import = "$0.strokeStyle = $1"]
pub fn set_stroke_style(&self, value: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.strokeStyle = $1"]
pub fn set_stroke_style_str(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CanvasGradient")]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.strokeStyle = $1"]
pub fn set_stroke_style_canvas_gradient(&self, value: &CanvasGradient) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CanvasPattern")]
    #[doc = "Setter for the `strokeStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.strokeStyle = $1"]
pub fn set_stroke_style_canvas_pattern(&self, value: &CanvasPattern) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fillStyle"]
pub fn fill_style(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    #[deprecated]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fillStyle = $1"]
pub fn set_fill_style(&self, value: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fillStyle = $1"]
pub fn set_fill_style_str(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CanvasGradient")]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fillStyle = $1"]
pub fn set_fill_style_canvas_gradient(&self, value: &CanvasGradient) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CanvasPattern")]
    #[doc = "Setter for the `fillStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fillStyle = $1"]
pub fn set_fill_style_canvas_pattern(&self, value: &CanvasPattern) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.filter"]
pub fn filter(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `filter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/filter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.filter = $1"]
pub fn set_filter(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `imageSmoothingEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.imageSmoothingEnabled"]
pub fn image_smoothing_enabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `imageSmoothingEnabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/imageSmoothingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.imageSmoothingEnabled = $1"]
pub fn set_image_smoothing_enabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lineWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.lineWidth"]
pub fn line_width(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `lineWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.lineWidth = $1"]
pub fn set_line_width(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lineCap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.lineCap"]
pub fn line_cap(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `lineCap` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineCap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.lineCap = $1"]
pub fn set_line_cap(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lineJoin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.lineJoin"]
pub fn line_join(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `lineJoin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineJoin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.lineJoin = $1"]
pub fn set_line_join(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `miterLimit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.miterLimit"]
pub fn miter_limit(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `miterLimit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/miterLimit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.miterLimit = $1"]
pub fn set_miter_limit(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lineDashOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.lineDashOffset"]
pub fn line_dash_offset(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `lineDashOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineDashOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.lineDashOffset = $1"]
pub fn set_line_dash_offset(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `shadowOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.shadowOffsetX"]
pub fn shadow_offset_x(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `shadowOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.shadowOffsetX = $1"]
pub fn set_shadow_offset_x(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `shadowOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.shadowOffsetY"]
pub fn shadow_offset_y(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `shadowOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowOffsetY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.shadowOffsetY = $1"]
pub fn set_shadow_offset_y(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `shadowBlur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.shadowBlur"]
pub fn shadow_blur(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `shadowBlur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowBlur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.shadowBlur = $1"]
pub fn set_shadow_blur(&self, value: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `shadowColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.shadowColor"]
pub fn shadow_color(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `shadowColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/shadowColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.shadowColor = $1"]
pub fn set_shadow_color(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `font` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.font"]
pub fn font(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `font` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/font)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.font = $1"]
pub fn set_font(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `textAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.textAlign"]
pub fn text_align(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `textAlign` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textAlign)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.textAlign = $1"]
pub fn set_text_align(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `textBaseline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.textBaseline"]
pub fn text_baseline(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `textBaseline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/textBaseline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.textBaseline = $1"]
pub fn set_text_baseline(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `drawWindow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawWindow($1, $2, $3, $4, $5, $6)"]
pub fn draw_window(&self, window: &Window, x: f64, y: f64, w: f64, h: f64, bg_color: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `drawWindow()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawWindow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Window`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawWindow($1, $2, $3, $4, $5, $6, $7)"]
pub fn draw_window_with_flags(&self, window: &Window, x: f64, y: f64, w: f64, h: f64, bg_color: &str, flags: u32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3)"]
pub fn draw_image_with_html_image_element(&self, image: &HtmlImageElement, dx: f64, dy: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgImageElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3)"]
pub fn draw_image_with_svg_image_element(&self, image: &SvgImageElement, dx: f64, dy: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3)"]
pub fn draw_image_with_html_canvas_element(&self, image: &HtmlCanvasElement, dx: f64, dy: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3)"]
pub fn draw_image_with_html_video_element(&self, image: &HtmlVideoElement, dx: f64, dy: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3)"]
pub fn draw_image_with_image_bitmap(&self, image: &ImageBitmap, dx: f64, dy: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `OffscreenCanvas`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3)"]
pub fn draw_image_with_offscreen_canvas(&self, image: &OffscreenCanvas, dx: f64, dy: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3)"]
pub fn draw_image_with_video_frame(&self, image: &VideoFrame, dx: f64, dy: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5)"]
pub fn draw_image_with_html_image_element_and_dw_and_dh(&self, image: &HtmlImageElement, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgImageElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5)"]
pub fn draw_image_with_svg_image_element_and_dw_and_dh(&self, image: &SvgImageElement, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5)"]
pub fn draw_image_with_html_canvas_element_and_dw_and_dh(&self, image: &HtmlCanvasElement, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5)"]
pub fn draw_image_with_html_video_element_and_dw_and_dh(&self, image: &HtmlVideoElement, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5)"]
pub fn draw_image_with_image_bitmap_and_dw_and_dh(&self, image: &ImageBitmap, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `OffscreenCanvas`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5)"]
pub fn draw_image_with_offscreen_canvas_and_dw_and_dh(&self, image: &OffscreenCanvas, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5)"]
pub fn draw_image_with_video_frame_and_dw_and_dh(&self, image: &VideoFrame, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlImageElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &HtmlImageElement, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgImageElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `SvgImageElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn draw_image_with_svg_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &SvgImageElement, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &HtmlCanvasElement, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn draw_image_with_html_video_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &HtmlVideoElement, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageBitmap`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn draw_image_with_image_bitmap_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &ImageBitmap, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `OffscreenCanvas`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn draw_image_with_offscreen_canvas_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &OffscreenCanvas, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `drawImage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawImage($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn draw_image_with_video_frame_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(&self, image: &VideoFrame, sx: f64, sy: f64, sw: f64, sh: f64, dx: f64, dy: f64, dw: f64, dh: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `beginPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/beginPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.beginPath()"]
pub fn begin_path(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.clip()"]
pub fn clip(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CanvasWindingRule")]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.clip($1)"]
pub fn clip_with_canvas_winding_rule(&self, winding: CanvasWindingRule) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Path2d")]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.clip($1)"]
pub fn clip_with_path_2d(&self, path: &Path2d) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    #[doc = "The `clip()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clip)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.clip($1, $2)"]
pub fn clip_with_path_2d_and_winding(&self, path: &Path2d, winding: CanvasWindingRule) { panic!("web-sys intrinsic") }
}
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fill()"]
pub fn fill(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CanvasWindingRule")]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fill($1)"]
pub fn fill_with_canvas_winding_rule(&self, winding: CanvasWindingRule) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Path2d")]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fill($1)"]
pub fn fill_with_path_2d(&self, path: &Path2d) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    #[doc = "The `fill()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fill)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fill($1, $2)"]
pub fn fill_with_path_2d_and_winding(&self, path: &Path2d, winding: CanvasWindingRule) { panic!("web-sys intrinsic") }
}
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.isPointInPath($1, $2)"]
pub fn is_point_in_path_with_f64(&self, x: f64, y: f64) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CanvasWindingRule")]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.isPointInPath($1, $2, $3)"]
pub fn is_point_in_path_with_f64_and_canvas_winding_rule(&self, x: f64, y: f64, winding: CanvasWindingRule) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Path2d")]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.isPointInPath($1, $2, $3)"]
pub fn is_point_in_path_with_path_2d_and_f64(&self, path: &Path2d, x: f64, y: f64) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CanvasWindingRule", feature = "Path2d",))]
    #[doc = "The `isPointInPath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInPath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `CanvasWindingRule`, `Path2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.isPointInPath($1, $2, $3, $4)"]
pub fn is_point_in_path_with_path_2d_and_f64_and_winding(&self, path: &Path2d, x: f64, y: f64, winding: CanvasWindingRule) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.isPointInStroke($1, $2)"]
pub fn is_point_in_stroke_with_x_and_y(&self, x: f64, y: f64) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Path2d")]
    #[doc = "The `isPointInStroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/isPointInStroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.isPointInStroke($1, $2, $3)"]
pub fn is_point_in_stroke_with_path_and_x_and_y(&self, path: &Path2d, x: f64, y: f64) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `stroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.stroke()"]
pub fn stroke(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Path2d")]
    #[doc = "The `stroke()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/stroke)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Path2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.stroke($1)"]
pub fn stroke_with_path(&self, path: &Path2d) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CanvasGradient")]
    #[doc = "The `createLinearGradient()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createLinearGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createLinearGradient($1, $2, $3, $4)"]
pub fn create_linear_gradient(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlImageElement",))]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlImageElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createPattern($1, $2)"]
pub fn create_pattern_with_html_image_element(&self, image: &HtmlImageElement, repetition: &str) -> Result<Option<CanvasPattern>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CanvasPattern", feature = "SvgImageElement",))]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `SvgImageElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createPattern($1, $2)"]
pub fn create_pattern_with_svg_image_element(&self, image: &SvgImageElement, repetition: &str) -> Result<Option<CanvasPattern>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlCanvasElement",))]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlCanvasElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createPattern($1, $2)"]
pub fn create_pattern_with_html_canvas_element(&self, image: &HtmlCanvasElement, repetition: &str) -> Result<Option<CanvasPattern>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CanvasPattern", feature = "HtmlVideoElement",))]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `HtmlVideoElement`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createPattern($1, $2)"]
pub fn create_pattern_with_html_video_element(&self, image: &HtmlVideoElement, repetition: &str) -> Result<Option<CanvasPattern>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CanvasPattern", feature = "ImageBitmap",))]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `ImageBitmap`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createPattern($1, $2)"]
pub fn create_pattern_with_image_bitmap(&self, image: &ImageBitmap, repetition: &str) -> Result<Option<CanvasPattern>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CanvasPattern", feature = "OffscreenCanvas",))]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `OffscreenCanvas`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createPattern($1, $2)"]
pub fn create_pattern_with_offscreen_canvas(&self, image: &OffscreenCanvas, repetition: &str) -> Result<Option<CanvasPattern>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "CanvasPattern", feature = "VideoFrame",))]
    #[doc = "The `createPattern()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `CanvasRenderingContext2d`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createPattern($1, $2)"]
pub fn create_pattern_with_video_frame(&self, image: &VideoFrame, repetition: &str) -> Result<Option<CanvasPattern>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CanvasGradient")]
    #[doc = "The `createRadialGradient()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createRadialGradient)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasGradient`, `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createRadialGradient($1, $2, $3, $4, $5, $6)"]
pub fn create_radial_gradient(&self, x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> Result<CanvasGradient, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `addHitRegion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.addHitRegion()"]
pub fn add_hit_region(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HitRegionOptions")]
    #[doc = "The `addHitRegion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/addHitRegion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `HitRegionOptions`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.addHitRegion($1)"]
pub fn add_hit_region_with_options(&self, options: &HitRegionOptions) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearHitRegions()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearHitRegions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.clearHitRegions()"]
pub fn clear_hit_regions(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeHitRegion()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/removeHitRegion)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.removeHitRegion($1)"]
pub fn remove_hit_region(&self, id: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `createImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createImageData($1, $2)"]
pub fn create_image_data_with_sw_and_sh(&self, sw: f64, sh: f64) -> Result<ImageData, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `createImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/createImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.createImageData($1)"]
pub fn create_image_data_with_imagedata(&self, imagedata: &ImageData) -> Result<ImageData, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `getImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.getImageData($1, $2, $3, $4)"]
pub fn get_image_data(&self, sx: f64, sy: f64, sw: f64, sh: f64) -> Result<ImageData, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.putImageData($1, $2, $3)"]
pub fn put_image_data(&self, imagedata: &ImageData, dx: f64, dy: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `putImageData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `ImageData`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.putImageData($1, $2, $3, $4, $5, $6, $7)"]
pub fn put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height(&self, imagedata: &ImageData, dx: f64, dy: f64, dirty_x: f64, dirty_y: f64, dirty_width: f64, dirty_height: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.arc($1, $2, $3, $4, $5)"]
pub fn arc(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `arc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.arc($1, $2, $3, $4, $5, $6)"]
pub fn arc_with_anticlockwise(&self, x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anticlockwise: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `arcTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/arcTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.arcTo($1, $2, $3, $4, $5)"]
pub fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `bezierCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/bezierCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.bezierCurveTo($1, $2, $3, $4, $5, $6)"]
pub fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `closePath()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/closePath)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.closePath()"]
pub fn close_path(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.ellipse($1, $2, $3, $4, $5, $6, $7)"]
pub fn ellipse(&self, x: f64, y: f64, radius_x: f64, radius_y: f64, rotation: f64, start_angle: f64, end_angle: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `ellipse()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/ellipse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.ellipse($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn ellipse_with_anticlockwise(&self, x: f64, y: f64, radius_x: f64, radius_y: f64, rotation: f64, start_angle: f64, end_angle: f64, anticlockwise: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `lineTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/lineTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.lineTo($1, $2)"]
pub fn line_to(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `moveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/moveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.moveTo($1, $2)"]
pub fn move_to(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `quadraticCurveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/quadraticCurveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.quadraticCurveTo($1, $2, $3, $4)"]
pub fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `rect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.rect($1, $2, $3, $4)"]
pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4)"]
pub fn round_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4, $5)"]
pub fn round_rect_with_f64(&self, x: f64, y: f64, w: f64, h: f64, radii: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `DomPointInit`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4, $5)"]
pub fn round_rect_with_dom_point_init(&self, x: f64, y: f64, w: f64, h: f64, radii: &DomPointInit) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4, $5)"]
pub fn round_rect_with_f64_sequence(&self, x: f64, y: f64, w: f64, h: f64, radii: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `roundRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.roundRect($1, $2, $3, $4, $5)"]
pub fn round_rect_with_dom_point_init_sequence(&self, x: f64, y: f64, w: f64, h: f64, radii: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getLineDash()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getLineDash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.getLineDash()"]
pub fn get_line_dash(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `setLineDash()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setLineDash)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.setLineDash($1)"]
pub fn set_line_dash(&self, segments: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/clearRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.clearRect($1, $2, $3, $4)"]
pub fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `fillRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fillRect($1, $2, $3, $4)"]
pub fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `strokeRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.strokeRect($1, $2, $3, $4)"]
pub fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.reset()"]
pub fn reset(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `restore()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/restore)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.restore()"]
pub fn restore(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `save()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/save)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.save()"]
pub fn save(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `fillText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fillText($1, $2, $3)"]
pub fn fill_text(&self, text: &str, x: f64, y: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `fillText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/fillText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.fillText($1, $2, $3, $4)"]
pub fn fill_text_with_max_width(&self, text: &str, x: f64, y: f64, max_width: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TextMetrics")]
    #[doc = "The `measureText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/measureText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `TextMetrics`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.measureText($1)"]
pub fn measure_text(&self, text: &str) -> Result<TextMetrics, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `strokeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.strokeText($1, $2, $3)"]
pub fn stroke_text(&self, text: &str, x: f64, y: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `strokeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/strokeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.strokeText($1, $2, $3, $4)"]
pub fn stroke_text_with_max_width(&self, text: &str, x: f64, y: f64, max_width: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomMatrix")]
    #[doc = "The `getTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/getTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `DomMatrix`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.getTransform()"]
pub fn get_transform(&self) -> Result<DomMatrix, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `resetTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/resetTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.resetTransform()"]
pub fn reset_transform(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `rotate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/rotate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.rotate($1)"]
pub fn rotate(&self, angle: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `scale()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/scale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.scale($1, $2)"]
pub fn scale(&self, x: f64, y: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.setTransform($1, $2, $3, $4, $5, $6)"]
pub fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.setTransform()"]
pub fn set_transform_with_default_dom_matrix_2d_init(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomMatrix2dInit")]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `DomMatrix2dInit`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.setTransform($1)"]
pub fn set_transform_with_dom_matrix_2d_init(&self, transform: &DomMatrix2dInit) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `transform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.transform($1, $2, $3, $4, $5, $6)"]
pub fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `translate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/translate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.translate($1, $2)"]
pub fn translate(&self, x: f64, y: f64) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `drawCustomFocusRing()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawCustomFocusRing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawCustomFocusRing($1)"]
pub fn draw_custom_focus_ring(&self, element: &Element) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "The `drawFocusIfNeeded()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawFocusIfNeeded)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`, `Element`*"]
impl CanvasRenderingContext2d {
    #[js::import = "$0.drawFocusIfNeeded($1)"]
pub fn draw_focus_if_needed(&self, element: &Element) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl CanvasRenderingContext2d {
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_DRAW_CARET` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_DRAW_CARET: u32 = 1u64 as u32;
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_DO_NOT_FLUSH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_DO_NOT_FLUSH: u32 = 2u64 as u32;
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_DRAW_VIEW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_DRAW_VIEW: u32 = 4u64 as u32;
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_USE_WIDGET_LAYERS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_USE_WIDGET_LAYERS: u32 = 8u64 as u32;
    #[doc = "The `CanvasRenderingContext2D.DRAWWINDOW_ASYNC_DECODE_IMAGES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasRenderingContext2d`*"]
    pub const DRAWWINDOW_ASYNC_DECODE_IMAGES: u32 = 16u64 as u32;
}
