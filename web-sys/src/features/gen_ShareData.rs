#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShareData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ShareData(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `files` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::import = "None"]
pub fn get_files(this: &ShareData) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `files` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::import = "None = $1"]
pub fn set_files(this: &ShareData, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `text` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::import = "None"]
pub fn get_text(this: &ShareData) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `text` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::import = "None = $1"]
pub fn set_text(this: &ShareData, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::import = "None"]
pub fn get_title(this: &ShareData) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `title` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::import = "None = $1"]
pub fn set_title(this: &ShareData, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::import = "None"]
pub fn get_url(this: &ShareData) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::import = "None = $1"]
pub fn set_url(this: &ShareData, val: &str) { panic!("web-sys intrinsic") }
impl ShareData {
    #[doc = "Construct a new `ShareData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShareData`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_files()` instead."]
#[js::import = "None"]
pub fn files(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_files(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_text()` instead."]
#[js::import = "None"]
pub fn text(&mut self, val: &str) -> &mut Self { self.set_text(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_title()` instead."]
#[js::import = "None"]
pub fn title(&mut self, val: &str) -> &mut Self { self.set_title(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_url()` instead."]
#[js::import = "None"]
pub fn url(&mut self, val: &str) -> &mut Self { self.set_url(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ShareData {
    fn default() -> Self {
        Self::new()
    }
}
