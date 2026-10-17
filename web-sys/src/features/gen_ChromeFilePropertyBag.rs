#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChromeFilePropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ChromeFilePropertyBag(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
#[js::import = "None"]
pub fn get_last_modified(this: &ChromeFilePropertyBag) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
#[js::import = "None = $1"]
pub fn set_last_modified(this: &ChromeFilePropertyBag, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
#[js::import = "None"]
pub fn get_type(this: &ChromeFilePropertyBag) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &ChromeFilePropertyBag, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `existenceCheck` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
#[js::import = "None"]
pub fn get_existence_check(this: &ChromeFilePropertyBag) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `existenceCheck` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
#[js::import = "None = $1"]
pub fn set_existence_check(this: &ChromeFilePropertyBag, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
#[js::import = "None"]
pub fn get_name(this: &ChromeFilePropertyBag) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &ChromeFilePropertyBag, val: &str) { panic!("web-sys intrinsic") }
impl ChromeFilePropertyBag {
    #[doc = "Construct a new `ChromeFilePropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
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
    #[deprecated = "Use `set_existence_check()` instead."]
#[js::import = "None"]
pub fn existence_check(&mut self, val: bool) -> &mut Self { self.set_existence_check(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ChromeFilePropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
