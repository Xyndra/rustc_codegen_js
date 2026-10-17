#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemAccess` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeySystemAccess(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `keySystem` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/keySystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
impl MediaKeySystemAccess {
    #[js::import = "$0.keySystem"]
pub fn key_system(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `createMediaKeys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/createMediaKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemAccess`*"]
impl MediaKeySystemAccess {
    #[js::import = "$0.createMediaKeys()"]
pub fn create_media_keys(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaKeySystemConfiguration")]
    #[doc = "The `getConfiguration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/getConfiguration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemAccess`, `MediaKeySystemConfiguration`*"]
impl MediaKeySystemAccess {
    #[js::import = "$0.getConfiguration()"]
pub fn get_configuration(&self) -> MediaKeySystemConfiguration { panic!("web-sys intrinsic") }
}
