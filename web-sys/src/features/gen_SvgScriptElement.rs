#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgScriptElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SvgScriptElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
impl SvgScriptElement {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
impl SvgScriptElement {
    #[js::import = "$0.type = $1"]
pub fn set_type(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
impl SvgScriptElement {
    #[js::import = "$0.crossOrigin"]
pub fn cross_origin(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `crossOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/crossOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgScriptElement`*"]
impl SvgScriptElement {
    #[js::import = "$0.crossOrigin = $1"]
pub fn set_cross_origin(&self, value: Option<&str>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgAnimatedString")]
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SVGScriptElement/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgAnimatedString`, `SvgScriptElement`*"]
impl SvgScriptElement {
    #[js::import = "$0.href"]
pub fn href(&self) -> SvgAnimatedString { panic!("web-sys intrinsic") }
}
