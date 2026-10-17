#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlShaderPrecisionFormat` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebGlShaderPrecisionFormat(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `rangeMin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
impl WebGlShaderPrecisionFormat {
    #[js::import = "$0.rangeMin"]
pub fn range_min(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rangeMax` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/rangeMax)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
impl WebGlShaderPrecisionFormat {
    #[js::import = "$0.rangeMax"]
pub fn range_max(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `precision` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLShaderPrecisionFormat/precision)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShaderPrecisionFormat`*"]
impl WebGlShaderPrecisionFormat {
    #[js::import = "$0.precision"]
pub fn precision(&self) -> i32 { panic!("web-sys intrinsic") }
}
