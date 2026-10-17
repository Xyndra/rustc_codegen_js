#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaintWorkletGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintWorkletGlobalScope`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PaintWorkletGlobalScope(pub ::wasm_bindgen::JsValue);
    #[doc = "The `registerPaint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintWorkletGlobalScope/registerPaint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintWorkletGlobalScope`*"]
impl PaintWorkletGlobalScope {
    #[js::import = "$0.registerPaint($1, $2)"]
pub fn register_paint(&self, name: &str, paint_ctor: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
