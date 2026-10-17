#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Cache` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Cache(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Request")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
impl Cache {
    #[js::import = "$0.add($1)"]
pub fn add_with_request(&self, request: &Request) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
impl Cache {
    #[js::import = "$0.add($1)"]
pub fn add_with_str(&self, request: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `addAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
impl Cache {
    #[js::import = "$0.addAll($1)"]
pub fn add_all_with_request_sequence(&self, requests: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `addAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/addAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
impl Cache {
    #[js::import = "$0.addAll($1)"]
pub fn add_all_with_str_sequence(&self, requests: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Request")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
impl Cache {
    #[js::import = "$0.delete($1)"]
pub fn delete_with_request(&self, request: &Request) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
impl Cache {
    #[js::import = "$0.delete($1)"]
pub fn delete_with_str(&self, request: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
impl Cache {
    #[js::import = "$0.delete($1, $2)"]
pub fn delete_with_request_and_options(&self, request: &Request, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
impl Cache {
    #[js::import = "$0.delete($1, $2)"]
pub fn delete_with_str_and_options(&self, request: &str, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
impl Cache {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Request")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
impl Cache {
    #[js::import = "$0.keys($1)"]
pub fn keys_with_request(&self, request: &Request) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
impl Cache {
    #[js::import = "$0.keys($1)"]
pub fn keys_with_str(&self, request: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
impl Cache {
    #[js::import = "$0.keys($1, $2)"]
pub fn keys_with_request_and_options(&self, request: &Request, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
impl Cache {
    #[js::import = "$0.keys($1, $2)"]
pub fn keys_with_str_and_options(&self, request: &str, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Request")]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
impl Cache {
    #[js::import = "$0.match($1)"]
pub fn match_with_request(&self, request: &Request) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
impl Cache {
    #[js::import = "$0.match($1)"]
pub fn match_with_str(&self, request: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
impl Cache {
    #[js::import = "$0.match($1, $2)"]
pub fn match_with_request_and_options(&self, request: &Request, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "The `match()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/match)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
impl Cache {
    #[js::import = "$0.match($1, $2)"]
pub fn match_with_str_and_options(&self, request: &str, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
impl Cache {
    #[js::import = "$0.matchAll()"]
pub fn match_all(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Request")]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`*"]
impl Cache {
    #[js::import = "$0.matchAll($1)"]
pub fn match_all_with_request(&self, request: &Request) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`*"]
impl Cache {
    #[js::import = "$0.matchAll($1)"]
pub fn match_all_with_str(&self, request: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CacheQueryOptions", feature = "Request",))]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`, `Request`*"]
impl Cache {
    #[js::import = "$0.matchAll($1, $2)"]
pub fn match_all_with_request_and_options(&self, request: &Request, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "The `matchAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/matchAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `CacheQueryOptions`*"]
impl Cache {
    #[js::import = "$0.matchAll($1, $2)"]
pub fn match_all_with_str_and_options(&self, request: &str, options: &CacheQueryOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Request", feature = "Response",))]
    #[doc = "The `put()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/put)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Request`, `Response`*"]
impl Cache {
    #[js::import = "$0.put($1, $2)"]
pub fn put_with_request(&self, request: &Request, response: &Response) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Response")]
    #[doc = "The `put()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Cache/put)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Cache`, `Response`*"]
impl Cache {
    #[js::import = "$0.put($1, $2)"]
pub fn put_with_str(&self, request: &str, response: &Response) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
