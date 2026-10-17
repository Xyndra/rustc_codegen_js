#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XPathNsResolver` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"]
#[js::opaque]
#[derive(Copy)]
pub struct XPathNsResolver(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `lookupNamespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"]
#[js::import = "None"]
pub fn get_lookup_namespace_uri(this: &XPathNsResolver) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lookupNamespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"]
#[js::import = "None = $1"]
pub fn set_lookup_namespace_uri(this: &XPathNsResolver, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
impl XPathNsResolver {
    #[doc = "Construct a new `XPathNsResolver`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XPathNsResolver`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_lookup_namespace_uri()` instead."]
#[js::import = "None"]
pub fn lookup_namespace_uri(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_lookup_namespace_uri(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for XPathNsResolver {
    fn default() -> Self {
        Self::new()
    }
}
