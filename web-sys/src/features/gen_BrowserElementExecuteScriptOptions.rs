#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserElementExecuteScriptOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BrowserElementExecuteScriptOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
#[js::import = "None"]
pub fn get_origin(this: &BrowserElementExecuteScriptOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
#[js::import = "None = $1"]
pub fn set_origin(this: &BrowserElementExecuteScriptOptions, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
#[js::import = "None"]
pub fn get_url(this: &BrowserElementExecuteScriptOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
#[js::import = "None = $1"]
pub fn set_url(this: &BrowserElementExecuteScriptOptions, val: Option<&str>) { panic!("web-sys intrinsic") }
impl BrowserElementExecuteScriptOptions {
    #[doc = "Construct a new `BrowserElementExecuteScriptOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementExecuteScriptOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_origin()` instead."]
#[js::import = "None"]
pub fn origin(&mut self, val: Option<&str>) -> &mut Self { self.set_origin(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_url()` instead."]
#[js::import = "None"]
pub fn url(&mut self, val: Option<&str>) -> &mut Self { self.set_url(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BrowserElementExecuteScriptOptions {
    fn default() -> Self {
        Self::new()
    }
}
