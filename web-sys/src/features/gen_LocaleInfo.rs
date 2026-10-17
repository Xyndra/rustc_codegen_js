#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LocaleInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
#[js::opaque]
#[derive(Copy)]
pub struct LocaleInfo(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
#[js::import = "None"]
pub fn get_direction(this: &LocaleInfo) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
#[js::import = "None = $1"]
pub fn set_direction(this: &LocaleInfo, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
#[js::import = "None"]
pub fn get_locale(this: &LocaleInfo) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
#[js::import = "None = $1"]
pub fn set_locale(this: &LocaleInfo, val: &str) { panic!("web-sys intrinsic") }
impl LocaleInfo {
    #[doc = "Construct a new `LocaleInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_direction()` instead."]
#[js::import = "None"]
pub fn direction(&mut self, val: &str) -> &mut Self { self.set_direction(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_locale()` instead."]
#[js::import = "None"]
pub fn locale(&mut self, val: &str) -> &mut Self { self.set_locale(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for LocaleInfo {
    fn default() -> Self {
        Self::new()
    }
}
