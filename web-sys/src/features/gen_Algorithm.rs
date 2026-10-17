#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Algorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Algorithm`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Algorithm(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Algorithm`*"]
#[js::import = "None"]
pub fn get_name(this: &Algorithm) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Algorithm`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &Algorithm, val: &str) { panic!("web-sys intrinsic") }
impl Algorithm {
    #[doc = "Construct a new `Algorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Algorithm`*"]
#[js::import = "None"]
pub fn new(name: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
}
