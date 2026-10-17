#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglDebugShaders` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglDebugShaders`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebglDebugShaders(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "WebGlShader")]
    #[doc = "The `getTranslatedShaderSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_debug_shaders/getTranslatedShaderSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlShader`, `WebglDebugShaders`*"]
impl WebglDebugShaders {
    #[js::import = "$0.getTranslatedShaderSource($1)"]
pub fn get_translated_shader_source(&self, shader: &WebGlShader) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
