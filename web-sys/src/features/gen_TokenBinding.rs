#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TokenBinding` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TokenBinding(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
#[js::import = "None"]
pub fn get_id(this: &TokenBinding) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &TokenBinding, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
#[js::import = "None"]
pub fn get_status(this: &TokenBinding) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
#[js::import = "None = $1"]
pub fn set_status(this: &TokenBinding, val: &str) { panic!("web-sys intrinsic") }
impl TokenBinding {
    #[doc = "Construct a new `TokenBinding`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TokenBinding`*"]
#[js::import = "None"]
pub fn new(status: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_status(status);
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
#[js::import = "None"]
pub fn id(&mut self, val: &str) -> &mut Self { self.set_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_status()` instead."]
#[js::import = "None"]
pub fn status(&mut self, val: &str) -> &mut Self { self.set_status(val) { panic!("web-sys intrinsic") }
        self
    }
}
