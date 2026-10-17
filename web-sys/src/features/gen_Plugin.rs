#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Plugin` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Plugin(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `description` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/description)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
impl Plugin {
    #[js::import = "$0.description"]
pub fn description(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `filename` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/filename)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
impl Plugin {
    #[js::import = "$0.filename"]
pub fn filename(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `version` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/version)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
impl Plugin {
    #[js::import = "$0.version"]
pub fn version(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
impl Plugin {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`*"]
impl Plugin {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MimeType")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
impl Plugin {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<MimeType> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MimeType")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Plugin/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
impl Plugin {
    #[js::import = "$0.namedItem($1)"]
pub fn named_item(&self, name: &str) -> Option<MimeType> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MimeType")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
impl Plugin {
    #[js::import = "$0.None($1)"]
pub fn get_with_index(&self, index: u32) -> Option<MimeType> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MimeType")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
impl Plugin {
    #[js::import = "$0.None($1)"]
pub fn get_with_name(&self, name: &str) -> Option<MimeType> { panic!("web-sys intrinsic") }
}
