#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConnStatusDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConnStatusDict(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
#[js::import = "None"]
pub fn get_status(this: &ConnStatusDict) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
#[js::import = "None = $1"]
pub fn set_status(this: &ConnStatusDict, val: &str) { panic!("web-sys intrinsic") }
impl ConnStatusDict {
    #[doc = "Construct a new `ConnStatusDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_status()` instead."]
#[js::import = "None"]
pub fn status(&mut self, val: &str) -> &mut Self { self.set_status(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConnStatusDict {
    fn default() -> Self {
        Self::new()
    }
}
