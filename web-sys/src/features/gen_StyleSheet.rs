#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StyleSheet` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StyleSheet(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
impl StyleSheet {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `href` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/href)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
impl StyleSheet {
    #[js::import = "$0.href"]
pub fn href(&self) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `ownerNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/ownerNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `StyleSheet`*"]
impl StyleSheet {
    #[js::import = "$0.ownerNode"]
pub fn owner_node(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `parentStyleSheet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/parentStyleSheet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
impl StyleSheet {
    #[js::import = "$0.parentStyleSheet"]
pub fn parent_style_sheet(&self) -> Option<StyleSheet> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `title` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/title)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
impl StyleSheet {
    #[js::import = "$0.title"]
pub fn title(&self) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaList")]
    #[doc = "Getter for the `media` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/media)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaList`, `StyleSheet`*"]
impl StyleSheet {
    #[js::import = "$0.media"]
pub fn media(&self) -> MediaList { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
impl StyleSheet {
    #[js::import = "$0.disabled"]
pub fn disabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `disabled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet/disabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StyleSheet`*"]
impl StyleSheet {
    #[js::import = "$0.disabled = $1"]
pub fn set_disabled(&self, value: bool) { panic!("web-sys intrinsic") }
}
