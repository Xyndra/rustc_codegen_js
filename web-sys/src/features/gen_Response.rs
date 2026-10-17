#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Response` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Response(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ResponseType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseType`*"]
impl Response {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ResponseType { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.url"]
pub fn url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `redirected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.redirected"]
pub fn redirected(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.status"]
pub fn status(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ok` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/ok)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.ok"]
pub fn ok(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/statusText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.statusText"]
pub fn status_text(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Headers")]
    #[doc = "Getter for the `headers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`, `Response`*"]
impl Response {
    #[js::import = "$0.headers"]
pub fn headers(&self) -> Headers { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bodyUsed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/bodyUsed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.bodyUsed"]
pub fn body_used(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableStream")]
    #[doc = "Getter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
impl Response {
    #[js::import = "$0.body"]
pub fn body(&self) -> Option<ReadableStream> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "new Response()"]
pub fn new() -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Response`*"]
impl Response {
    #[js::import = "new Response($0)"]
pub fn new_with_opt_blob(body: Option<&Blob>) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "new Response($0)"]
pub fn new_with_opt_buffer_source(body: Option<&::js_sys::Object>) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "new Response($0)"]
pub fn new_with_opt_u8_array(body: Option<&mut [u8]>) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "new Response($0)"]
pub fn new_with_opt_js_u8_array(body: Option<&::js_sys::Uint8Array>,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FormData")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `Response`*"]
impl Response {
    #[js::import = "new Response($0)"]
pub fn new_with_opt_form_data(body: Option<&FormData>) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "UrlSearchParams")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `UrlSearchParams`*"]
impl Response {
    #[js::import = "new Response($0)"]
pub fn new_with_opt_url_search_params(body: Option<&UrlSearchParams>,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "new Response($0)"]
pub fn new_with_opt_str(body: Option<&str>) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableStream")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Response`*"]
impl Response {
    #[js::import = "new Response($0)"]
pub fn new_with_opt_readable_stream(body: Option<&ReadableStream>) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Blob", feature = "ResponseInit",))]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Response`, `ResponseInit`*"]
impl Response {
    #[js::import = "new Response($0, $1)"]
pub fn new_with_opt_blob_and_init(body: Option<&Blob>, init: &ResponseInit,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ResponseInit")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
impl Response {
    #[js::import = "new Response($0, $1)"]
pub fn new_with_opt_buffer_source_and_init(body: Option<&::js_sys::Object>, init: &ResponseInit,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ResponseInit")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
impl Response {
    #[js::import = "new Response($0, $1)"]
pub fn new_with_opt_u8_array_and_init(body: Option<&mut [u8]>, init: &ResponseInit,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ResponseInit")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
impl Response {
    #[js::import = "new Response($0, $1)"]
pub fn new_with_opt_js_u8_array_and_init(body: Option<&::js_sys::Uint8Array>, init: &ResponseInit,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "FormData", feature = "ResponseInit",))]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `Response`, `ResponseInit`*"]
impl Response {
    #[js::import = "new Response($0, $1)"]
pub fn new_with_opt_form_data_and_init(body: Option<&FormData>, init: &ResponseInit,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ResponseInit", feature = "UrlSearchParams",))]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`, `UrlSearchParams`*"]
impl Response {
    #[js::import = "new Response($0, $1)"]
pub fn new_with_opt_url_search_params_and_init(body: Option<&UrlSearchParams>, init: &ResponseInit,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ResponseInit")]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`, `ResponseInit`*"]
impl Response {
    #[js::import = "new Response($0, $1)"]
pub fn new_with_opt_str_and_init(body: Option<&str>, init: &ResponseInit,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ReadableStream", feature = "ResponseInit",))]
    #[doc = "The `new Response(..)` constructor, creating a new instance of `Response`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/Response)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Response`, `ResponseInit`*"]
impl Response {
    #[js::import = "new Response($0, $1)"]
pub fn new_with_opt_readable_stream_and_init(body: Option<&ReadableStream>, init: &ResponseInit,) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.clone()"]
pub fn clone(&self) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `error()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/error_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "Response.error"]
pub fn error() -> Response { panic!("web-sys intrinsic") }
}
    #[doc = "The `redirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "Response.redirect"]
pub fn redirect(url: &str) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `redirect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/redirect_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "Response.redirect"]
pub fn redirect_with_status(url: &str, status: u16) -> Result<Response, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `arrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/arrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.arrayBuffer()"]
pub fn array_buffer(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `blob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.blob()"]
pub fn blob(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `formData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/formData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.formData()"]
pub fn form_data(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `json()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/json)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.json()"]
pub fn json(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `text()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Response/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Response`*"]
impl Response {
    #[js::import = "$0.text()"]
pub fn text(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
