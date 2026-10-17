#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdleRequestOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdleRequestOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`*"]
#[js::import = "None"]
pub fn get_timeout(this: &IdleRequestOptions) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timeout` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`*"]
#[js::import = "None = $1"]
pub fn set_timeout(this: &IdleRequestOptions, val: u32) { panic!("web-sys intrinsic") }
impl IdleRequestOptions {
    #[doc = "Construct a new `IdleRequestOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_timeout()` instead."]
#[js::import = "None"]
pub fn timeout(&mut self, val: u32) -> &mut Self { self.set_timeout(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for IdleRequestOptions {
    fn default() -> Self {
        Self::new()
    }
}
