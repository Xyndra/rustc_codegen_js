#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomStringList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomStringList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
impl DomStringList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `contains()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/contains)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
impl DomStringList {
    #[js::import = "$0.contains($1)"]
pub fn contains(&self, string: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMStringList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
impl DomStringList {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomStringList`*"]
impl DomStringList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
