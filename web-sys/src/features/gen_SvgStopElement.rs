#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgStopElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStopElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgStopElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgStopElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedNumber")]
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGStopElement/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedNumber`, `SvgStopElement`*"]
impl SvgStopElement {
    #[js::import = "$0.offset"]
pub fn offset(&self) -> SvgAnimatedNumber { panic!("web-sys intrinsic") }
}
