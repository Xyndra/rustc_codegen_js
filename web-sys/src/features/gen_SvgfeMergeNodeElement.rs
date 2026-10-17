#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgfeMergeNodeElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeNodeElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgfeMergeNodeElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgfeMergeNodeElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `in1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGFEMergeNodeElement/in1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgfeMergeNodeElement`*"]
impl SvgfeMergeNodeElement {
    #[js::import = "$0.in1"]
pub fn in1(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
