#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MimeTypeArray` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeTypeArray`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MimeTypeArray(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeTypeArray`*"]
impl MimeTypeArray {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MimeType")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
impl MimeTypeArray {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<MimeType> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MimeType")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeTypeArray/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
impl MimeTypeArray {
    #[js::import = "$0.namedItem($1)"]
pub fn named_item(&self, name: &str) -> Option<MimeType> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MimeType")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
impl MimeTypeArray {
    #[js::import = "$0.None($1)"]
pub fn get_with_index(&self, index: u32) -> Option<MimeType> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MimeType")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `MimeTypeArray`*"]
impl MimeTypeArray {
    #[js::import = "$0.None($1)"]
pub fn get_with_name(&self, name: &str) -> Option<MimeType> { panic!("web-sys intrinsic") }
}
