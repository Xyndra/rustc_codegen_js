#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HalfOpenInfoDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HalfOpenInfoDict(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `speculative` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
#[js::import = "None"]
pub fn get_speculative(this: &HalfOpenInfoDict) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `speculative` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
#[js::import = "None = $1"]
pub fn set_speculative(this: &HalfOpenInfoDict, val: bool) { panic!("web-sys intrinsic") }
impl HalfOpenInfoDict {
    #[doc = "Construct a new `HalfOpenInfoDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_speculative()` instead."]
#[js::import = "None"]
pub fn speculative(&mut self, val: bool) -> &mut Self { self.set_speculative(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for HalfOpenInfoDict {
    fn default() -> Self {
        Self::new()
    }
}
