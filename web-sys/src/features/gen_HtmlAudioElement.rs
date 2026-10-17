#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlAudioElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlAudioElement(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new HtmlAudioElement(..)` constructor, creating a new instance of `HtmlAudioElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/HTMLAudioElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
impl Audio {
    #[js::import = "new Audio()"]
pub fn new() -> Result<HtmlAudioElement, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new HtmlAudioElement(..)` constructor, creating a new instance of `HtmlAudioElement`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLAudioElement/HTMLAudioElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAudioElement`*"]
impl Audio {
    #[js::import = "new Audio($0)"]
pub fn new_with_src(src: &str) -> Result<HtmlAudioElement, JsValue> { panic!("web-sys intrinsic") }
}
