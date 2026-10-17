#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlSerializer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlSerializer`*"]
#[js::opaque]
#[derive(Copy)]
pub struct XmlSerializer(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new XmlSerializer(..)` constructor, creating a new instance of `XmlSerializer`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/XMLSerializer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlSerializer`*"]
impl XmlSerializer {
    #[js::import = "new XMLSerializer()"]
pub fn new() -> Result<XmlSerializer, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `serializeToString()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLSerializer/serializeToString)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XmlSerializer`*"]
impl XmlSerializer {
    #[js::import = "$0.serializeToString($1)"]
pub fn serialize_to_string(&self, root: &Node) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
