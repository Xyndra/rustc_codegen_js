#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CacheStorage` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CacheStorage(pub ::wasm_bindgen::JsValue);
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
impl CacheStorage {
    #[js::import = "$0.delete($1)"]
pub fn delete(&self, cache_name: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
impl CacheStorage {
    #[js::import = "$0.has($1)"]
pub fn has(&self, cache_name: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
impl CacheStorage {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Request")]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`, `Request`*"]
impl CacheStorage {
    #[js::import = "$0.match($1)"]
pub fn match_with_request(&self, request: &Request) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
impl CacheStorage {
    #[js::import = "$0.match($1)"]
pub fn match_with_str(&self, request: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`, `CacheStorage`, `Request`*"]
impl CacheStorage {
    #[js::import = "$0.match($1, $2)"]
pub fn match_with_request_and_options(&self, request: &Request, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`, `CacheStorage`*"]
impl CacheStorage {
    #[js::import = "$0.match($1, $2)"]
pub fn match_with_str_and_options(&self, request: &str, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`*"]
impl CacheStorage {
    #[js::import = "$0.open($1)"]
pub fn open(&self, cache_name: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
