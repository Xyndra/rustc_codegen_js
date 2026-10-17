#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgsvgElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgsvgElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `x` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/x)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.x"]
pub fn x(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `y` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/y)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.y"]
pub fn y(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `width` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/width)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.width"]
pub fn width(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedLength")]
    #[doc = "Getter for the `height` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/height)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedLength`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.height"]
pub fn height(&self) -> SvgAnimatedLength { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `useCurrentView` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/useCurrentView)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.useCurrentView"]
pub fn use_current_view(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `currentScale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.currentScale"]
pub fn current_scale(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `currentScale` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentScale)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.currentScale = $1"]
pub fn set_current_scale(&self, value: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPoint")]
    #[doc = "Getter for the `currentTranslate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/currentTranslate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.currentTranslate"]
pub fn current_translate(&self) -> SvgPoint { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/viewBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.viewBox"]
pub fn view_box(&self) -> SvgAnimatedRect { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.preserveAspectRatio"]
pub fn preserve_aspect_ratio(&self) -> SvgAnimatedPreserveAspectRatio { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.zoomAndPan"]
pub fn zoom_and_pan(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `zoomAndPan` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/zoomAndPan)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.zoomAndPan = $1"]
pub fn set_zoom_and_pan(&self, value: u16) { panic!("web-sys intrinsic") }
}
    #[doc = "The `animationsPaused()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/animationsPaused)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.animationsPaused()"]
pub fn animations_paused(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAngle")]
    #[doc = "The `createSVGAngle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAngle`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.createSVGAngle()"]
pub fn create_svg_angle(&self) -> SvgAngle { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgLength")]
    #[doc = "The `createSVGLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgLength`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.createSVGLength()"]
pub fn create_svg_length(&self) -> SvgLength { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgMatrix")]
    #[doc = "The `createSVGMatrix()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.createSVGMatrix()"]
pub fn create_svg_matrix(&self) -> SvgMatrix { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgNumber")]
    #[doc = "The `createSVGNumber()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgNumber`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.createSVGNumber()"]
pub fn create_svg_number(&self) -> SvgNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPoint")]
    #[doc = "The `createSVGPoint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGPoint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPoint`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.createSVGPoint()"]
pub fn create_svg_point(&self) -> SvgPoint { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgRect")]
    #[doc = "The `createSVGRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgRect`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.createSVGRect()"]
pub fn create_svg_rect(&self) -> SvgRect { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgTransform")]
    #[doc = "The `createSVGTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgTransform`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.createSVGTransform()"]
pub fn create_svg_transform(&self) -> SvgTransform { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "SvgMatrix", feature = "SvgTransform",))]
    #[doc = "The `createSVGTransformFromMatrix()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/createSVGTransformFromMatrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgMatrix`, `SvgTransform`, `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.createSVGTransformFromMatrix($1)"]
pub fn create_svg_transform_from_matrix(&self, matrix: &SvgMatrix) -> SvgTransform { panic!("web-sys intrinsic") }
}
    #[doc = "The `deselectAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/deselectAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.deselectAll()"]
pub fn deselect_all(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `forceRedraw()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/forceRedraw)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.forceRedraw()"]
pub fn force_redraw(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getCurrentTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getCurrentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.getCurrentTime()"]
pub fn get_current_time(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getElementById()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/getElementById)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.getElementById($1)"]
pub fn get_element_by_id(&self, element_id: &str) -> Option<Element> { panic!("web-sys intrinsic") }
}
    #[doc = "The `pauseAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/pauseAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.pauseAnimations()"]
pub fn pause_animations(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setCurrentTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/setCurrentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.setCurrentTime($1)"]
pub fn set_current_time(&self, seconds: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `suspendRedraw()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/suspendRedraw)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.suspendRedraw($1)"]
pub fn suspend_redraw(&self, max_wait_milliseconds: u32) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `unpauseAnimations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unpauseAnimations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.unpauseAnimations()"]
pub fn unpause_animations(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `unsuspendRedraw()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedraw)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.unsuspendRedraw($1)"]
pub fn unsuspend_redraw(&self, suspend_handle_id: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `unsuspendRedrawAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSVGElement/unsuspendRedrawAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
impl SvgsvgElement {
    #[js::import = "$0.unsuspendRedrawAll()"]
pub fn unsuspend_redraw_all(&self) { panic!("web-sys intrinsic") }
}
impl SvgsvgElement {
    #[doc = "The `SVGSVGElement.SVG_ZOOMANDPAN_UNKNOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub const SVG_ZOOMANDPAN_UNKNOWN: u16 = 0i64 as u16;
    #[doc = "The `SVGSVGElement.SVG_ZOOMANDPAN_DISABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub const SVG_ZOOMANDPAN_DISABLE: u16 = 1u64 as u16;
    #[doc = "The `SVGSVGElement.SVG_ZOOMANDPAN_MAGNIFY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgsvgElement`*"]
    pub const SVG_ZOOMANDPAN_MAGNIFY: u16 = 2u64 as u16;
}
