#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgPathElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgPathElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgPathSegList")]
    #[doc = "Getter for the `pathSegList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/pathSegList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathElement`, `SvgPathSegList`*"]
impl SvgPathElement {
    #[js::import = "$0.pathSegList"]
pub fn path_seg_list(&self) -> SvgPathSegList { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgPathSegList")]
    #[doc = "Getter for the `animatedPathSegList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/animatedPathSegList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathElement`, `SvgPathSegList`*"]
impl SvgPathElement {
    #[js::import = "$0.animatedPathSegList"]
pub fn animated_path_seg_list(&self) -> SvgPathSegList { panic!("web-sys intrinsic") }
}
    #[doc = "The `getPathSegAtLength()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathElement/getPathSegAtLength)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgPathElement`*"]
impl SvgPathElement {
    #[js::import = "$0.getPathSegAtLength($1)"]
pub fn get_path_seg_at_length(&self, distance: f32) -> u32 { panic!("web-sys intrinsic") }
}
