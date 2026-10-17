#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsLookupDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DnsLookupDict(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `address` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
#[js::import = "None"]
pub fn get_address(this: &DnsLookupDict) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `address` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
#[js::import = "None = $1"]
pub fn set_address(this: &DnsLookupDict, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `answer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
#[js::import = "None"]
pub fn get_answer(this: &DnsLookupDict) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `answer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
#[js::import = "None = $1"]
pub fn set_answer(this: &DnsLookupDict, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
#[js::import = "None"]
pub fn get_error(this: &DnsLookupDict) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
#[js::import = "None = $1"]
pub fn set_error(this: &DnsLookupDict, val: &str) { panic!("web-sys intrinsic") }
impl DnsLookupDict {
    #[doc = "Construct a new `DnsLookupDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_address()` instead."]
#[js::import = "None"]
pub fn address(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_address(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_answer()` instead."]
#[js::import = "None"]
pub fn answer(&mut self, val: bool) -> &mut Self { self.set_answer(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_error()` instead."]
#[js::import = "None"]
pub fn error(&mut self, val: &str) -> &mut Self { self.set_error(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DnsLookupDict {
    fn default() -> Self {
        Self::new()
    }
}
