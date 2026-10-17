#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XsltProcessor` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
#[js::opaque]
#[derive(Copy)]
pub struct XsltProcessor(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new XsltProcessor(..)` constructor, creating a new instance of `XsltProcessor`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/XSLTProcessor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
impl XsltProcessor {
    #[js::import = "new XSLTProcessor()"]
pub fn new() -> Result<XsltProcessor, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearParameters()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/clearParameters)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
impl XsltProcessor {
    #[js::import = "$0.clearParameters()"]
pub fn clear_parameters(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `importStylesheet()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/importStylesheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `XsltProcessor`*"]
impl XsltProcessor {
    #[js::import = "$0.importStylesheet($1)"]
pub fn import_stylesheet(&self, style: &Node) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `removeParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/removeParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
impl XsltProcessor {
    #[js::import = "$0.removeParameter($1, $2)"]
pub fn remove_parameter(&self, namespace_uri: &str, local_name: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `reset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/reset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
impl XsltProcessor {
    #[js::import = "$0.reset()"]
pub fn reset(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/setParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XsltProcessor`*"]
impl XsltProcessor {
    #[js::import = "$0.setParameter($1, $2, $3)"]
pub fn set_parameter(&self, namespace_uri: &str, local_name: &str, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "Node",))]
    #[doc = "The `transformToDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Node`, `XsltProcessor`*"]
impl XsltProcessor {
    #[js::import = "$0.transformToDocument($1)"]
pub fn transform_to_document(&self, source: &Node) -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "DocumentFragment", feature = "Node",))]
    #[doc = "The `transformToFragment()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XSLTProcessor/transformToFragment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentFragment`, `Node`, `XsltProcessor`*"]
impl XsltProcessor {
    #[js::import = "$0.transformToFragment($1, $2)"]
pub fn transform_to_fragment(&self, source: &Node, output: &Document) -> Result<DocumentFragment, JsValue> { panic!("web-sys intrinsic") }
}
