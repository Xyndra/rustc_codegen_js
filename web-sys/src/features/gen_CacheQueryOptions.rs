#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CacheQueryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CacheQueryOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `cacheName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::import = "None"]
pub fn get_cache_name(this: &CacheQueryOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cacheName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::import = "None = $1"]
pub fn set_cache_name(this: &CacheQueryOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ignoreMethod` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::import = "None"]
pub fn get_ignore_method(this: &CacheQueryOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ignoreMethod` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::import = "None = $1"]
pub fn set_ignore_method(this: &CacheQueryOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ignoreSearch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::import = "None"]
pub fn get_ignore_search(this: &CacheQueryOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ignoreSearch` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::import = "None = $1"]
pub fn set_ignore_search(this: &CacheQueryOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ignoreVary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::import = "None"]
pub fn get_ignore_vary(this: &CacheQueryOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ignoreVary` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::import = "None = $1"]
pub fn set_ignore_vary(this: &CacheQueryOptions, val: bool) { panic!("web-sys intrinsic") }
impl CacheQueryOptions {
    #[doc = "Construct a new `CacheQueryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheQueryOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_cache_name()` instead."]
#[js::import = "None"]
pub fn cache_name(&mut self, val: &str) -> &mut Self { self.set_cache_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ignore_method()` instead."]
#[js::import = "None"]
pub fn ignore_method(&mut self, val: bool) -> &mut Self { self.set_ignore_method(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ignore_search()` instead."]
#[js::import = "None"]
pub fn ignore_search(&mut self, val: bool) -> &mut Self { self.set_ignore_search(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ignore_vary()` instead."]
#[js::import = "None"]
pub fn ignore_vary(&mut self, val: bool) -> &mut Self { self.set_ignore_vary(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for CacheQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
