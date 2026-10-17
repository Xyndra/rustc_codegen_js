#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `mediaText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`*"]
impl MediaList {
    #[js::import = "$0.mediaText"]
pub fn media_text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `mediaText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/mediaText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`*"]
impl MediaList {
    #[js::import = "$0.mediaText = $1"]
pub fn set_media_text(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`*"]
impl MediaList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `appendMedium()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/appendMedium)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`*"]
impl MediaList {
    #[js::import = "$0.appendMedium($1)"]
pub fn append_medium(&self, new_medium: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `deleteMedium()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/deleteMedium)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`*"]
impl MediaList {
    #[js::import = "$0.deleteMedium($1)"]
pub fn delete_medium(&self, old_medium: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaList/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`*"]
impl MediaList {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`*"]
impl MediaList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
