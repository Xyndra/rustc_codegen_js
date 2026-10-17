#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChromeWorker` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChromeWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeWorker`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ChromeWorker(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new ChromeWorker(..)` constructor, creating a new instance of `ChromeWorker`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ChromeWorker/ChromeWorker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeWorker`*"]
impl ChromeWorker {
    #[js::import = "new ChromeWorker($0)"]
pub fn new(script_url: &str) -> Result<ChromeWorker, JsValue> { panic!("web-sys intrinsic") }
}
