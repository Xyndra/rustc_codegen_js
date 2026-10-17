#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CssNamespaceRule` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CssNamespaceRule(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/namespaceURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
impl CssNamespaceRule {
    #[js::import = "$0.namespaceURI"]
pub fn namespace_uri(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/prefix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssNamespaceRule`*"]
impl CssNamespaceRule {
    #[js::import = "$0.prefix"]
pub fn prefix(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
