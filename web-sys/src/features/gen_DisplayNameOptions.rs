#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayNameOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DisplayNameOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
#[js::import = "None"]
pub fn get_keys(this: &DisplayNameOptions) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
#[js::import = "None = $1"]
pub fn set_keys(this: &DisplayNameOptions, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
#[js::import = "None"]
pub fn get_style(this: &DisplayNameOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
#[js::import = "None = $1"]
pub fn set_style(this: &DisplayNameOptions, val: &str) { panic!("web-sys intrinsic") }
impl DisplayNameOptions {
    #[doc = "Construct a new `DisplayNameOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_keys()` instead."]
#[js::import = "None"]
pub fn keys(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_keys(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_style()` instead."]
#[js::import = "None"]
pub fn style(&mut self, val: &str) -> &mut Self { self.set_style(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DisplayNameOptions {
    fn default() -> Self {
        Self::new()
    }
}
