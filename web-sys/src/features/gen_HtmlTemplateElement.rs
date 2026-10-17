#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlTemplateElement` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlTemplateElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlTemplateElement(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DocumentFragment")]
    #[doc = "Getter for the `content` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLTemplateElement/content)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentFragment`, `HtmlTemplateElement`*"]
impl HtmlTemplateElement {
    #[js::import = "$0.content"]
pub fn content(&self) -> DocumentFragment { panic!("web-sys intrinsic") }
}
