#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XmlHttpRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct XmlHttpRequest(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `onreadystatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/onreadystatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.onreadystatechange"]
pub fn onreadystatechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onreadystatechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/onreadystatechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.onreadystatechange = $1"]
pub fn set_onreadystatechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `readyState` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/readyState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.readyState"]
pub fn ready_state(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/timeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.timeout"]
pub fn timeout(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/timeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.timeout = $1"]
pub fn set_timeout(&self, value: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/withCredentials)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.withCredentials"]
pub fn with_credentials(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/withCredentials)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.withCredentials = $1"]
pub fn set_with_credentials(&self, value: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "XmlHttpRequestUpload")]
    #[doc = "Getter for the `upload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/upload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`, `XmlHttpRequestUpload`*"]
impl XmlHttpRequest {
    #[js::import = "$0.upload"]
pub fn upload(&self) -> Result<XmlHttpRequestUpload, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `responseURL` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseURL)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.responseURL"]
pub fn response_url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.status"]
pub fn status(&self) -> Result<u16, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/statusText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.statusText"]
pub fn status_text(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "XmlHttpRequestResponseType")]
    #[doc = "Getter for the `responseType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`, `XmlHttpRequestResponseType`*"]
impl XmlHttpRequest {
    #[js::import = "$0.responseType"]
pub fn response_type(&self) -> XmlHttpRequestResponseType { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "XmlHttpRequestResponseType")]
    #[doc = "Setter for the `responseType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`, `XmlHttpRequestResponseType`*"]
impl XmlHttpRequest {
    #[js::import = "$0.responseType = $1"]
pub fn set_response_type(&self, value: XmlHttpRequestResponseType) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `response` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.response"]
pub fn response(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `responseText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.responseText"]
pub fn response_text(&self) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `responseXML` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/responseXML)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.responseXML"]
pub fn response_xml(&self) -> Result<Option<Document>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new XmlHttpRequest(..)` constructor, creating a new instance of `XmlHttpRequest`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/XMLHttpRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "new XMLHttpRequest()"]
pub fn new() -> Result<XmlHttpRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new XmlHttpRequest(..)` constructor, creating a new instance of `XmlHttpRequest`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/XMLHttpRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "new XMLHttpRequest($0)"]
pub fn new_with_ignored(ignored: &str) -> Result<XmlHttpRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `abort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/abort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.abort()"]
pub fn abort(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAllResponseHeaders()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getAllResponseHeaders)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.getAllResponseHeaders()"]
pub fn get_all_response_headers(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getResponseHeader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/getResponseHeader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.getResponseHeader($1)"]
pub fn get_response_header(&self, header: &str) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.open($1, $2)"]
pub fn open(&self, method: &str, url: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.open($1, $2, $3)"]
pub fn open_with_async(&self, method: &str, url: &str, r#async: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.open($1, $2, $3, $4)"]
pub fn open_with_async_and_user(&self, method: &str, url: &str, r#async: bool, user: Option<&str>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.open($1, $2, $3, $4, $5)"]
pub fn open_with_async_and_user_and_password(&self, method: &str, url: &str, r#async: bool, user: Option<&str>, password: Option<&str>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `overrideMimeType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/overrideMimeType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.overrideMimeType($1)"]
pub fn override_mime_type(&self, mime: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send()"]
pub fn send(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send($1)"]
pub fn send_with_opt_document(&self, body: Option<&Document>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send($1)"]
pub fn send_with_opt_blob(&self, body: Option<&Blob>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send($1)"]
pub fn send_with_opt_buffer_source(&self, body: Option<&::js_sys::Object>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send($1)"]
pub fn send_with_opt_u8_array(&self, body: Option<&[u8]>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send($1)"]
pub fn send_with_opt_js_u8_array(&self, body: Option<&::js_sys::Uint8Array>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FormData")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send($1)"]
pub fn send_with_opt_form_data(&self, body: Option<&FormData>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "UrlSearchParams")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`, `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send($1)"]
pub fn send_with_opt_url_search_params(&self, body: Option<&UrlSearchParams>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send($1)"]
pub fn send_with_opt_str(&self, body: Option<&str>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableStream")]
    #[doc = "The `send()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/send)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.send($1)"]
pub fn send_with_opt_readable_stream(&self, body: Option<&ReadableStream>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setRequestHeader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/setRequestHeader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
impl XmlHttpRequest {
    #[js::import = "$0.setRequestHeader($1, $2)"]
pub fn set_request_header(&self, header: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl XmlHttpRequest {
    #[doc = "The `XMLHttpRequest.UNSENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub const UNSENT: u16 = 0i64 as u16;
    #[doc = "The `XMLHttpRequest.OPENED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub const OPENED: u16 = 1u64 as u16;
    #[doc = "The `XMLHttpRequest.HEADERS_RECEIVED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub const HEADERS_RECEIVED: u16 = 2u64 as u16;
    #[doc = "The `XMLHttpRequest.LOADING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub const LOADING: u16 = 3u64 as u16;
    #[doc = "The `XMLHttpRequest.DONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XmlHttpRequest`*"]
    pub const DONE: u16 = 4u64 as u16;
}
