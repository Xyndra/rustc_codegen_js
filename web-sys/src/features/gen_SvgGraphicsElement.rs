#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgGraphicsElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgGraphicsElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedTransformList")]
    #[doc = "Getter for the `transform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/transform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedTransformList`, `SvgGraphicsElement`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.transform"]
pub fn transform(&self) -> SvgAnimatedTransformList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `nearestViewportElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/nearestViewportElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.nearestViewportElement"]
pub fn nearest_viewport_element(&self) -> Option<SvgElement> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `farthestViewportElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/farthestViewportElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.farthestViewportElement"]
pub fn farthest_viewport_element(&self) -> Option<SvgElement> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/requiredFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.requiredFeatures"]
pub fn required_features(&self) -> SvgStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredExtensions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/requiredExtensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.requiredExtensions"]
pub fn required_extensions(&self) -> SvgStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `systemLanguage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/systemLanguage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgStringList`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.systemLanguage"]
pub fn system_language(&self) -> SvgStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgRect")]
    #[doc = "The `getBBox()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgRect`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.getBBox()"]
pub fn get_b_box(&self) -> Result<SvgRect, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "SvgBoundingBoxOptions", feature = "SvgRect",))]
    #[doc = "The `getBBox()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getBBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`, `SvgGraphicsElement`, `SvgRect`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.getBBox($1)"]
pub fn get_b_box_with_a_options(&self, a_options: &SvgBoundingBoxOptions) -> Result<SvgRect, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgMatrix")]
    #[doc = "The `getCTM()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getCTM)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.getCTM()"]
pub fn get_ctm(&self) -> Option<SvgMatrix> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgMatrix")]
    #[doc = "The `getScreenCTM()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getScreenCTM)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.getScreenCTM()"]
pub fn get_screen_ctm(&self) -> Option<SvgMatrix> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgMatrix")]
    #[doc = "The `getTransformToElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/getTransformToElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`, `SvgMatrix`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.getTransformToElement($1)"]
pub fn get_transform_to_element(&self, element: &SvgGraphicsElement) -> Result<SvgMatrix, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasExtension()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGGraphicsElement/hasExtension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgGraphicsElement`*"]
impl SvgGraphicsElement {
    #[js::import = "$0.hasExtension($1)"]
pub fn has_extension(&self, extension: &str) -> bool { panic!("web-sys intrinsic") }
}
