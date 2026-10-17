#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomParser` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomParser`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomParser(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new DomParser(..)` constructor, creating a new instance of `DomParser`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser/DOMParser)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomParser`*"]
impl DomParser {
    #[js::import = "new DOMParser()"]
pub fn new() -> Result<DomParser, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "SupportedType",))]
    #[doc = "The `parseFromString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMParser/parseFromString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomParser`, `SupportedType`*"]
impl DomParser {
    #[js::import = "$0.parseFromString($1, $2)"]
pub fn parse_from_string(&self, str: &str, type_: SupportedType) -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
