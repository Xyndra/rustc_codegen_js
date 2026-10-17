#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClipboardItem` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ClipboardItem(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PresentationStyle")]
    #[doc = "Getter for the `presentationStyle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/presentationStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`, `PresentationStyle`*"]
impl ClipboardItem {
    #[js::import = "$0.presentationStyle"]
pub fn presentation_style(&self) -> PresentationStyle { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `types` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/types)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
impl ClipboardItem {
    #[js::import = "$0.types"]
pub fn types(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ClipboardItem(..)` constructor, creating a new instance of `ClipboardItem`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
impl ClipboardItem {
    #[js::import = "new ClipboardItem($0)"]
pub fn new_with_record_from_str_to_str_promise(items: &::js_sys::Object,) -> Result<ClipboardItem, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new ClipboardItem(..)` constructor, creating a new instance of `ClipboardItem`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
impl ClipboardItem {
    #[js::import = "new ClipboardItem($0)"]
pub fn new_with_record_from_str_to_blob_promise(items: &::js_sys::Object,) -> Result<ClipboardItem, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ClipboardItemOptions")]
    #[doc = "The `new ClipboardItem(..)` constructor, creating a new instance of `ClipboardItem`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`, `ClipboardItemOptions`*"]
impl ClipboardItem {
    #[js::import = "new ClipboardItem($0, $1)"]
pub fn new_with_record_from_str_to_str_promise_and_options(items: &::js_sys::Object, options: &ClipboardItemOptions,) -> Result<ClipboardItem, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ClipboardItemOptions")]
    #[doc = "The `new ClipboardItem(..)` constructor, creating a new instance of `ClipboardItem`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/ClipboardItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`, `ClipboardItemOptions`*"]
impl ClipboardItem {
    #[js::import = "new ClipboardItem($0, $1)"]
pub fn new_with_record_from_str_to_blob_promise_and_options(items: &::js_sys::Object, options: &ClipboardItemOptions,) -> Result<ClipboardItem, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/getType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
impl ClipboardItem {
    #[js::import = "$0.getType($1)"]
pub fn get_type(&self, type_: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `supports()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ClipboardItem/supports_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClipboardItem`*"]
impl ClipboardItem {
    #[js::import = "ClipboardItem.supports"]
pub fn supports(type_: &str) -> bool { panic!("web-sys intrinsic") }
}
