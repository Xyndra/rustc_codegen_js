#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnectionElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HttpConnectionElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None"]
pub fn get_active(this: &HttpConnectionElement) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None = $1"]
pub fn set_active(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `halfOpens` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None"]
pub fn get_half_opens(this: &HttpConnectionElement) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `halfOpens` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None = $1"]
pub fn set_half_opens(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None"]
pub fn get_host(this: &HttpConnectionElement) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None = $1"]
pub fn set_host(this: &HttpConnectionElement, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `idle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None"]
pub fn get_idle(this: &HttpConnectionElement) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `idle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None = $1"]
pub fn set_idle(this: &HttpConnectionElement, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None"]
pub fn get_port(this: &HttpConnectionElement) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None = $1"]
pub fn set_port(this: &HttpConnectionElement, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `spdy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None"]
pub fn get_spdy(this: &HttpConnectionElement) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `spdy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None = $1"]
pub fn set_spdy(this: &HttpConnectionElement, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None"]
pub fn get_ssl(this: &HttpConnectionElement) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None = $1"]
pub fn set_ssl(this: &HttpConnectionElement, val: bool) { panic!("web-sys intrinsic") }
impl HttpConnectionElement {
    #[doc = "Construct a new `HttpConnectionElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
#[js::import = "None"]
pub fn active(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_active(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_half_opens()` instead."]
#[js::import = "None"]
pub fn half_opens(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_half_opens(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_host()` instead."]
#[js::import = "None"]
pub fn host(&mut self, val: &str) -> &mut Self { self.set_host(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_idle()` instead."]
#[js::import = "None"]
pub fn idle(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_idle(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_port()` instead."]
#[js::import = "None"]
pub fn port(&mut self, val: u32) -> &mut Self { self.set_port(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_spdy()` instead."]
#[js::import = "None"]
pub fn spdy(&mut self, val: bool) -> &mut Self { self.set_spdy(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ssl()` instead."]
#[js::import = "None"]
pub fn ssl(&mut self, val: bool) -> &mut Self { self.set_ssl(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for HttpConnectionElement {
    fn default() -> Self {
        Self::new()
    }
}
