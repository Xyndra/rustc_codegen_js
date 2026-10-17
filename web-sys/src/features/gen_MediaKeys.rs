#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeys` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeys(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `keySystem` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/keySystem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
impl MediaKeys {
    #[js::import = "$0.keySystem"]
pub fn key_system(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaKeySession")]
    #[doc = "The `createSession()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeys`*"]
impl MediaKeys {
    #[js::import = "$0.createSession()"]
pub fn create_session(&self) -> Result<MediaKeySession, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "MediaKeySession", feature = "MediaKeySessionType",))]
    #[doc = "The `createSession()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/createSession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeySessionType`, `MediaKeys`*"]
impl MediaKeys {
    #[js::import = "$0.createSession($1)"]
pub fn create_session_with_session_type(&self, session_type: MediaKeySessionType) -> Result<MediaKeySession, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getStatusForPolicy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
impl MediaKeys {
    #[js::import = "$0.getStatusForPolicy()"]
pub fn get_status_for_policy(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaKeysPolicy")]
    #[doc = "The `getStatusForPolicy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/getStatusForPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`, `MediaKeysPolicy`*"]
impl MediaKeys {
    #[js::import = "$0.getStatusForPolicy($1)"]
pub fn get_status_for_policy_with_policy(&self, policy: &MediaKeysPolicy) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `setServerCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
impl MediaKeys {
    #[js::import = "$0.setServerCertificate($1)"]
pub fn set_server_certificate_with_buffer_source(&self, server_certificate: &::js_sys::Object) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `setServerCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
impl MediaKeys {
    #[js::import = "$0.setServerCertificate($1)"]
pub fn set_server_certificate_with_u8_array(&self, server_certificate: &mut [u8]) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `setServerCertificate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeys/setServerCertificate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeys`*"]
impl MediaKeys {
    #[js::import = "$0.setServerCertificate($1)"]
pub fn set_server_certificate_with_js_u8_array(&self, server_certificate: &::js_sys::Uint8Array) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
