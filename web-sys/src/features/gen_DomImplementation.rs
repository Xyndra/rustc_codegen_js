#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomImplementation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomImplementation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomImplementation(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Document")]
    #[doc = "The `createDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
impl DomImplementation {
    #[js::import = "$0.createDocument($1, $2)"]
pub fn create_document(&self, namespace: Option<&str>, qualified_name: &str) -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Document", feature = "DocumentType",))]
    #[doc = "The `createDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DocumentType`, `DomImplementation`*"]
impl DomImplementation {
    #[js::import = "$0.createDocument($1, $2, $3)"]
pub fn create_document_with_doctype(&self, namespace: Option<&str>, qualified_name: &str, doctype: Option<&DocumentType>) -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DocumentType")]
    #[doc = "The `createDocumentType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createDocumentType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentType`, `DomImplementation`*"]
impl DomImplementation {
    #[js::import = "$0.createDocumentType($1, $2, $3)"]
pub fn create_document_type(&self, qualified_name: &str, public_id: &str, system_id: &str) -> Result<DocumentType, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "The `createHTMLDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
impl DomImplementation {
    #[js::import = "$0.createHTMLDocument()"]
pub fn create_html_document(&self) -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "The `createHTMLDocument()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/createHTMLDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `DomImplementation`*"]
impl DomImplementation {
    #[js::import = "$0.createHTMLDocument($1)"]
pub fn create_html_document_with_title(&self, title: &str) -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `hasFeature()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMImplementation/hasFeature)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomImplementation`*"]
impl DomImplementation {
    #[js::import = "$0.hasFeature()"]
pub fn has_feature(&self) -> bool { panic!("web-sys intrinsic") }
}
