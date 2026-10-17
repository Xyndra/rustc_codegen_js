#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Request` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Request(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `method` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/method)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.method"]
pub fn method(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `url` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/url)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.url"]
pub fn url(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Headers")]
    #[doc = "Getter for the `headers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`, `Request`*"]
impl Request {
    #[js::import = "$0.headers"]
pub fn headers(&self) -> Headers { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RequestDestination")]
    #[doc = "Getter for the `destination` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/destination)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestDestination`*"]
impl Request {
    #[js::import = "$0.destination"]
pub fn destination(&self) -> RequestDestination { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.referrer"]
pub fn referrer(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReferrerPolicy")]
    #[doc = "Getter for the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/referrerPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReferrerPolicy`, `Request`*"]
impl Request {
    #[js::import = "$0.referrerPolicy"]
pub fn referrer_policy(&self) -> ReferrerPolicy { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RequestMode")]
    #[doc = "Getter for the `mode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/mode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestMode`*"]
impl Request {
    #[js::import = "$0.mode"]
pub fn mode(&self) -> RequestMode { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Getter for the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestCredentials`*"]
impl Request {
    #[js::import = "$0.credentials"]
pub fn credentials(&self) -> RequestCredentials { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RequestCache")]
    #[doc = "Getter for the `cache` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/cache)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestCache`*"]
impl Request {
    #[js::import = "$0.cache"]
pub fn cache(&self) -> RequestCache { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RequestRedirect")]
    #[doc = "Getter for the `redirect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/redirect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestRedirect`*"]
impl Request {
    #[js::import = "$0.redirect"]
pub fn redirect(&self) -> RequestRedirect { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/integrity)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.integrity"]
pub fn integrity(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "AbortSignal")]
    #[doc = "Getter for the `signal` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/signal)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `Request`*"]
impl Request {
    #[js::import = "$0.signal"]
pub fn signal(&self) -> AbortSignal { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bodyUsed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/bodyUsed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.bodyUsed"]
pub fn body_used(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ReadableStream")]
    #[doc = "Getter for the `body` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/body)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `Request`*"]
impl Request {
    #[js::import = "$0.body"]
pub fn body(&self) -> Option<ReadableStream> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "new Request($0)"]
pub fn new_with_request(input: &Request) -> Result<Request, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "new Request($0)"]
pub fn new_with_str(input: &str) -> Result<Request, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RequestInit")]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
impl Request {
    #[js::import = "new Request($0, $1)"]
pub fn new_with_request_and_init(input: &Request, init: &RequestInit,) -> Result<Request, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RequestInit")]
    #[doc = "The `new Request(..)` constructor, creating a new instance of `Request`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestInit`*"]
impl Request {
    #[js::import = "new Request($0, $1)"]
pub fn new_with_str_and_init(input: &str, init: &RequestInit) -> Result<Request, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.clone()"]
pub fn clone(&self) -> Result<Request, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `arrayBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/arrayBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.arrayBuffer()"]
pub fn array_buffer(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `blob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/blob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.blob()"]
pub fn blob(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `formData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/formData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.formData()"]
pub fn form_data(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `json()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/json)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.json()"]
pub fn json(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `text()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Request/text)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`*"]
impl Request {
    #[js::import = "$0.text()"]
pub fn text(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
