#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlDocument` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlDocument`*"]
#[js::opaque]
#[derive(Copy)]
pub struct XmlDocument(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `async` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/async)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlDocument`*"]
impl XmlDocument {
    #[js::import = "$0.async"]
pub fn r(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `async` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/async)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlDocument`*"]
impl XmlDocument {
    #[js::import = "$0.async = $1"]
pub fn set_async(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLDocument/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlDocument`*"]
impl XmlDocument {
    #[js::import = "$0.load($1)"]
pub fn load(&self, url: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
