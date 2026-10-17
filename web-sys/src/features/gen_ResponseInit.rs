#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResponseInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ResponseInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
#[js::import = "None"]
pub fn get_headers(this: &ResponseInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
#[js::import = "None = $1"]
pub fn set_headers(this: &ResponseInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
#[js::import = "None"]
pub fn get_status(this: &ResponseInit) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
#[js::import = "None = $1"]
pub fn set_status(this: &ResponseInit, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
#[js::import = "None"]
pub fn get_status_text(this: &ResponseInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
#[js::import = "None = $1"]
pub fn set_status_text(this: &ResponseInit, val: &str) { panic!("web-sys intrinsic") }
impl ResponseInit {
    #[doc = "Construct a new `ResponseInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_headers()` instead."]
#[js::import = "None"]
pub fn headers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_headers(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_status()` instead."]
#[js::import = "None"]
pub fn status(&mut self, val: u16) -> &mut Self { self.set_status(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_status_text()` instead."]
#[js::import = "None"]
pub fn status_text(&mut self, val: &str) -> &mut Self { self.set_status_text(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ResponseInit {
    fn default() -> Self {
        Self::new()
    }
}
