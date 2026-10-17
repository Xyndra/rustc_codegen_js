#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgSymbolElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgSymbolElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgSymbolElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedRect")]
    #[doc = "Getter for the `viewBox` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/viewBox)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedRect`, `SvgSymbolElement`*"]
impl SvgSymbolElement {
    #[js::import = "$0.viewBox"]
pub fn view_box(&self) -> SvgAnimatedRect { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedPreserveAspectRatio")]
    #[doc = "Getter for the `preserveAspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/preserveAspectRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedPreserveAspectRatio`, `SvgSymbolElement`*"]
impl SvgSymbolElement {
    #[js::import = "$0.preserveAspectRatio"]
pub fn preserve_aspect_ratio(&self) -> SvgAnimatedPreserveAspectRatio { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredFeatures` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredFeatures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
impl SvgSymbolElement {
    #[js::import = "$0.requiredFeatures"]
pub fn required_features(&self) -> SvgStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `requiredExtensions` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/requiredExtensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
impl SvgSymbolElement {
    #[js::import = "$0.requiredExtensions"]
pub fn required_extensions(&self) -> SvgStringList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgStringList")]
    #[doc = "Getter for the `systemLanguage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/systemLanguage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStringList`, `SvgSymbolElement`*"]
impl SvgSymbolElement {
    #[js::import = "$0.systemLanguage"]
pub fn system_language(&self) -> SvgStringList { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasExtension()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGSymbolElement/hasExtension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgSymbolElement`*"]
impl SvgSymbolElement {
    #[js::import = "$0.hasExtension($1)"]
pub fn has_extension(&self, extension: &str) -> bool { panic!("web-sys intrinsic") }
}
