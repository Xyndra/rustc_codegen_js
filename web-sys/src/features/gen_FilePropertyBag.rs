#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FilePropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FilePropertyBag(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
#[js::import = "None"]
pub fn get_last_modified(this: &FilePropertyBag) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
#[js::import = "None = $1"]
pub fn set_last_modified(this: &FilePropertyBag, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
#[js::import = "None"]
pub fn get_type(this: &FilePropertyBag) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &FilePropertyBag, val: &str) { panic!("web-sys intrinsic") }
impl FilePropertyBag {
    #[doc = "Construct a new `FilePropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_last_modified()` instead."]
#[js::import = "None"]
pub fn last_modified(&mut self, val: f64) -> &mut Self { self.set_last_modified(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: &str) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FilePropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
