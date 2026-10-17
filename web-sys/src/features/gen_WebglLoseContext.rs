#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebglLoseContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglLoseContext`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebglLoseContext(pub ::wasm_bindgen::JsValue);
    #[doc = "The `loseContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/loseContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglLoseContext`*"]
impl WebglLoseContext {
    #[js::import = "$0.loseContext()"]
pub fn lose_context(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `restoreContext()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_lose_context/restoreContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebglLoseContext`*"]
impl WebglLoseContext {
    #[js::import = "$0.restoreContext()"]
pub fn restore_context(&self) { panic!("web-sys intrinsic") }
}
