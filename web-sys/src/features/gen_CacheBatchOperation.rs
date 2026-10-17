#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CacheBatchOperation` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CacheBatchOperation(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "Get the `options` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `CacheQueryOptions`*"]
#[js::import = "None"]
pub fn get_options(this: &CacheBatchOperation) -> Option<CacheQueryOptions> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "Change the `options` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `CacheQueryOptions`*"]
#[js::import = "None = $1"]
pub fn set_options(this: &CacheBatchOperation, val: &CacheQueryOptions) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Request")]
    #[doc = "Get the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*"]
#[js::import = "None"]
pub fn get_request(this: &CacheBatchOperation) -> Option<Request> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Request")]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*"]
#[js::import = "None = $1"]
pub fn set_request(this: &CacheBatchOperation, val: &Request) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Response")]
    #[doc = "Get the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*"]
#[js::import = "None"]
pub fn get_response(this: &CacheBatchOperation) -> Option<Response> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Response")]
    #[doc = "Change the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*"]
#[js::import = "None = $1"]
pub fn set_response(this: &CacheBatchOperation, val: &Response) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
#[js::import = "None"]
pub fn get_type(this: &CacheBatchOperation) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &CacheBatchOperation, val: &str) { panic!("web-sys intrinsic") }
impl CacheBatchOperation {
    #[doc = "Construct a new `CacheBatchOperation`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "CacheQueryOptions")]
    #[deprecated = "Use `set_options()` instead."]
#[js::import = "None"]
pub fn options(&mut self, val: &CacheQueryOptions) -> &mut Self { self.set_options(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "Request")]
    #[deprecated = "Use `set_request()` instead."]
#[js::import = "None"]
pub fn request(&mut self, val: &Request) -> &mut Self { self.set_request(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "Response")]
    #[deprecated = "Use `set_response()` instead."]
#[js::import = "None"]
pub fn response(&mut self, val: &Response) -> &mut Self { self.set_response(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: &str) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for CacheBatchOperation {
    fn default() -> Self {
        Self::new()
    }
}
