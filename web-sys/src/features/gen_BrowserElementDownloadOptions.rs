#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserElementDownloadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BrowserElementDownloadOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
#[js::import = "None"]
pub fn get_filename(this: &BrowserElementDownloadOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
#[js::import = "None = $1"]
pub fn set_filename(this: &BrowserElementDownloadOptions, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
#[js::import = "None"]
pub fn get_referrer(this: &BrowserElementDownloadOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
#[js::import = "None = $1"]
pub fn set_referrer(this: &BrowserElementDownloadOptions, val: Option<&str>) { panic!("web-sys intrinsic") }
impl BrowserElementDownloadOptions {
    #[doc = "Construct a new `BrowserElementDownloadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_filename()` instead."]
#[js::import = "None"]
pub fn filename(&mut self, val: Option<&str>) -> &mut Self { self.set_filename(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_referrer()` instead."]
#[js::import = "None"]
pub fn referrer(&mut self, val: Option<&str>) -> &mut Self { self.set_referrer(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BrowserElementDownloadOptions {
    fn default() -> Self {
        Self::new()
    }
}
