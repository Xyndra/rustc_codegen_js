#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySession` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeySession(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "MediaKeyError")]
    #[doc = "Getter for the `error` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/error)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyError`, `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.error"]
pub fn error(&self) -> Option<MediaKeyError> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sessionId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/sessionId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.sessionId"]
pub fn session_id(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/expiration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.expiration"]
pub fn expiration(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.closed"]
pub fn closed(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaKeyStatusMap")]
    #[doc = "Getter for the `keyStatuses` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/keyStatuses)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`, `MediaKeyStatusMap`*"]
impl MediaKeySession {
    #[js::import = "$0.keyStatuses"]
pub fn key_statuses(&self) -> MediaKeyStatusMap { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeystatuseschange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.onkeystatuseschange"]
pub fn onkeystatuseschange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeystatuseschange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.onkeystatuseschange = $1"]
pub fn set_onkeystatuseschange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.close()"]
pub fn close(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `generateRequest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.generateRequest($1, $2)"]
pub fn generate_request_with_buffer_source(&self, init_data_type: &str, init_data: &::js_sys::Object) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `generateRequest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.generateRequest($1, $2)"]
pub fn generate_request_with_u8_array(&self, init_data_type: &str, init_data: &mut [u8]) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `generateRequest()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.generateRequest($1, $2)"]
pub fn generate_request_with_js_u8_array(&self, init_data_type: &str, init_data: &::js_sys::Uint8Array) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.load($1)"]
pub fn load(&self, session_id: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `remove()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/remove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.remove()"]
pub fn remove(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.update($1)"]
pub fn update_with_buffer_source(&self, response: &::js_sys::Object) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.update($1)"]
pub fn update_with_u8_array(&self, response: &mut [u8]) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySession`*"]
impl MediaKeySession {
    #[js::import = "$0.update($1)"]
pub fn update_with_js_u8_array(&self, response: &::js_sys::Uint8Array) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
