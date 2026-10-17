#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MimeType` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MimeType(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `description` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/description)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`*"]
impl MimeType {
    #[js::import = "$0.description"]
pub fn description(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Plugin")]
    #[doc = "Getter for the `enabledPlugin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/enabledPlugin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`, `Plugin`*"]
impl MimeType {
    #[js::import = "$0.enabledPlugin"]
pub fn enabled_plugin(&self) -> Option<Plugin> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `suffixes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/suffixes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`*"]
impl MimeType {
    #[js::import = "$0.suffixes"]
pub fn suffixes(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MimeType`*"]
impl MimeType {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
