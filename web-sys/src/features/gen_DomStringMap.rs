#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomStringMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomStringMap(pub ::wasm_bindgen::JsValue);
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`*"]
impl DomStringMap {
    #[js::import = "$0.None($1)"]
pub fn get(&self, name: &str) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing setter. As in the literal Javascript `this[key] = value`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`*"]
impl DomStringMap {
    #[js::import = "$0.None($1, $2)"]
pub fn set(&self, name: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing deleter. As in the literal Javascript `delete this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringMap`*"]
impl DomStringMap {
    #[js::import = "$0.None($1)"]
pub fn delete(&self, name: &str) { panic!("web-sys intrinsic") }
}
