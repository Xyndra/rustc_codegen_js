#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RequestInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RequestInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None"]
pub fn get_body(this: &RequestInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `body` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_body(this: &RequestInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestCache")]
    #[doc = "Get the `cache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCache`, `RequestInit`*"]
#[js::import = "None"]
pub fn get_cache(this: &RequestInit) -> Option<RequestCache> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestCache")]
    #[doc = "Change the `cache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCache`, `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_cache(this: &RequestInit, val: RequestCache) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Get the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `RequestInit`*"]
#[js::import = "None"]
pub fn get_credentials(this: &RequestInit) -> Option<RequestCredentials> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_credentials(this: &RequestInit, val: RequestCredentials) { panic!("web-sys intrinsic") }
    #[doc = "Get the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None"]
pub fn get_headers(this: &RequestInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_headers(this: &RequestInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None"]
pub fn get_integrity(this: &RequestInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `integrity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_integrity(this: &RequestInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `method` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None"]
pub fn get_method(this: &RequestInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `method` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_method(this: &RequestInit, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestMode`*"]
#[js::import = "None"]
pub fn get_mode(this: &RequestInit) -> Option<RequestMode> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestMode`*"]
#[js::import = "None = $1"]
pub fn set_mode(this: &RequestInit, val: RequestMode) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ObserverCallback")]
    #[doc = "Get the `observe` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`, `RequestInit`*"]
#[js::import = "None"]
pub fn get_observe(this: &RequestInit) -> Option<ObserverCallback> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ObserverCallback")]
    #[doc = "Change the `observe` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ObserverCallback`, `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_observe(this: &RequestInit, val: &ObserverCallback) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestRedirect")]
    #[doc = "Get the `redirect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestRedirect`*"]
#[js::import = "None"]
pub fn get_redirect(this: &RequestInit) -> Option<RequestRedirect> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestRedirect")]
    #[doc = "Change the `redirect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `RequestRedirect`*"]
#[js::import = "None = $1"]
pub fn set_redirect(this: &RequestInit, val: RequestRedirect) { panic!("web-sys intrinsic") }
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None"]
pub fn get_referrer(this: &RequestInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_referrer(this: &RequestInit, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ReferrerPolicy")]
    #[doc = "Get the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReferrerPolicy`, `RequestInit`*"]
#[js::import = "None"]
pub fn get_referrer_policy(this: &RequestInit) -> Option<ReferrerPolicy> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ReferrerPolicy")]
    #[doc = "Change the `referrerPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReferrerPolicy`, `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_referrer_policy(this: &RequestInit, val: ReferrerPolicy) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `RequestInit`*"]
#[js::import = "None"]
pub fn get_signal(this: &RequestInit) -> Option<AbortSignal> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `RequestInit`*"]
#[js::import = "None = $1"]
pub fn set_signal(this: &RequestInit, val: Option<&AbortSignal>) { panic!("web-sys intrinsic") }
impl RequestInit {
    #[doc = "Construct a new `RequestInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_body()` instead."]
#[js::import = "None"]
pub fn body(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self { self.set_body(val.unwrap_or(&::wasm_bindgen::JsValue::NULL)) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RequestCache")]
    #[deprecated = "Use `set_cache()` instead."]
#[js::import = "None"]
pub fn cache(&mut self, val: RequestCache) -> &mut Self { self.set_cache(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RequestCredentials")]
    #[deprecated = "Use `set_credentials()` instead."]
#[js::import = "None"]
pub fn credentials(&mut self, val: RequestCredentials) -> &mut Self { self.set_credentials(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_headers()` instead."]
#[js::import = "None"]
pub fn headers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_headers(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_integrity()` instead."]
#[js::import = "None"]
pub fn integrity(&mut self, val: &str) -> &mut Self { self.set_integrity(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_method()` instead."]
#[js::import = "None"]
pub fn method(&mut self, val: &str) -> &mut Self { self.set_method(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RequestMode")]
    #[deprecated = "Use `set_mode()` instead."]
#[js::import = "None"]
pub fn mode(&mut self, val: RequestMode) -> &mut Self { self.set_mode(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ObserverCallback")]
    #[deprecated = "Use `set_observe()` instead."]
#[js::import = "None"]
pub fn observe(&mut self, val: &ObserverCallback) -> &mut Self { self.set_observe(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RequestRedirect")]
    #[deprecated = "Use `set_redirect()` instead."]
#[js::import = "None"]
pub fn redirect(&mut self, val: RequestRedirect) -> &mut Self { self.set_redirect(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_referrer()` instead."]
#[js::import = "None"]
pub fn referrer(&mut self, val: &str) -> &mut Self { self.set_referrer(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ReferrerPolicy")]
    #[deprecated = "Use `set_referrer_policy()` instead."]
#[js::import = "None"]
pub fn referrer_policy(&mut self, val: ReferrerPolicy) -> &mut Self { self.set_referrer_policy(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[deprecated = "Use `set_signal()` instead."]
#[js::import = "None"]
pub fn signal(&mut self, val: Option<&AbortSignal>) -> &mut Self { self.set_signal(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RequestInit {
    fn default() -> Self {
        Self::new()
    }
}
