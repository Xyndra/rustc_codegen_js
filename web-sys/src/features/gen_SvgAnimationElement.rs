#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgAnimationElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgAnimationElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `targetElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/targetElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
impl SvgAnimationElement {
    #[js::import = "$0.targetElement"]
pub fn target_element(&self) -> Option<SvgElement> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
impl SvgAnimationElement {
    #[js::import = "$0.requiredFeatures"]
pub fn required_features(&self) -> SvgStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredExtensions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/requiredExtensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
impl SvgAnimationElement {
    #[js::import = "$0.requiredExtensions"]
pub fn required_extensions(&self) -> SvgStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `systemLanguage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/systemLanguage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`, `SvgStringList`*"]
impl SvgAnimationElement {
    #[js::import = "$0.systemLanguage"]
pub fn system_language(&self) -> SvgStringList { panic!("web-sys intrinsic") }
}
    #[doc = "The `beginElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
impl SvgAnimationElement {
    #[js::import = "$0.beginElement()"]
pub fn begin_element(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `beginElementAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/beginElementAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
impl SvgAnimationElement {
    #[js::import = "$0.beginElementAt($1)"]
pub fn begin_element_at(&self, offset: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `endElement()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
impl SvgAnimationElement {
    #[js::import = "$0.endElement()"]
pub fn end_element(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `endElementAt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/endElementAt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
impl SvgAnimationElement {
    #[js::import = "$0.endElementAt($1)"]
pub fn end_element_at(&self, offset: f32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getCurrentTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getCurrentTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
impl SvgAnimationElement {
    #[js::import = "$0.getCurrentTime()"]
pub fn get_current_time(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getSimpleDuration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getSimpleDuration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
impl SvgAnimationElement {
    #[js::import = "$0.getSimpleDuration()"]
pub fn get_simple_duration(&self) -> Result<f32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getStartTime()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/getStartTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
impl SvgAnimationElement {
    #[js::import = "$0.getStartTime()"]
pub fn get_start_time(&self) -> Result<f32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasExtension()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGAnimationElement/hasExtension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimationElement`*"]
impl SvgAnimationElement {
    #[js::import = "$0.hasExtension($1)"]
pub fn has_extension(&self, extension: &str) -> bool { panic!("web-sys intrinsic") }
}
