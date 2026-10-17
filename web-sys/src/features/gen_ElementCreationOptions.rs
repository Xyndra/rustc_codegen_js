#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ElementCreationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ElementCreationOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `is` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
#[js::import = "None"]
pub fn get_is(this: &ElementCreationOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `is` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_is(this: &ElementCreationOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `pseudo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
#[js::import = "None"]
pub fn get_pseudo(this: &ElementCreationOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `pseudo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
#[js::import = "None = $1"]
pub fn set_pseudo(this: &ElementCreationOptions, val: &str) { panic!("web-sys intrinsic") }
impl ElementCreationOptions {
    #[doc = "Construct a new `ElementCreationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementCreationOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_is()` instead."]
#[js::import = "None"]
pub fn is(&mut self, val: &str) -> &mut Self { self.set_is(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_pseudo()` instead."]
#[js::import = "None"]
pub fn pseudo(&mut self, val: &str) -> &mut Self { self.set_pseudo(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ElementCreationOptions {
    fn default() -> Self {
        Self::new()
    }
}
