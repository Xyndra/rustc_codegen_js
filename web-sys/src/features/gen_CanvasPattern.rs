#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CanvasPattern` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CanvasPattern(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "SvgMatrix")]
    #[doc = "The `setTransform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CanvasPattern/setTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CanvasPattern`, `SvgMatrix`*"]
impl CanvasPattern {
    #[js::import = "$0.setTransform($1)"]
pub fn set_transform(&self, matrix: &SvgMatrix) { panic!("web-sys intrinsic") }
}
