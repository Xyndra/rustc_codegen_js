#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ElementDefinitionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ElementDefinitionOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `extends` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
#[js::import = "None"]
pub fn get_extends(this: &ElementDefinitionOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `extends` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
#[js::import = "None = $1"]
pub fn set_extends(this: &ElementDefinitionOptions, val: &str) { panic!("web-sys intrinsic") }
impl ElementDefinitionOptions {
    #[doc = "Construct a new `ElementDefinitionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ElementDefinitionOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_extends()` instead."]
#[js::import = "None"]
pub fn extends(&mut self, val: &str) -> &mut Self { self.set_extends(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ElementDefinitionOptions {
    fn default() -> Self {
        Self::new()
    }
}
