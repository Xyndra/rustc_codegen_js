#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeDistantLightElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeDistantLightElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgfeDistantLightElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `azimuth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement/azimuth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDistantLightElement`*"]
impl SvgfeDistantLightElement {
    #[js::import = "$0.azimuth"]
pub fn azimuth(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `elevation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEDistantLightElement/elevation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgfeDistantLightElement`*"]
impl SvgfeDistantLightElement {
    #[js::import = "$0.elevation"]
pub fn elevation(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
