#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleCounterError` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConsoleCounterError(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
#[js::import = "None"]
pub fn get_error(this: &ConsoleCounterError) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
#[js::import = "None = $1"]
pub fn set_error(this: &ConsoleCounterError, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
#[js::import = "None"]
pub fn get_label(this: &ConsoleCounterError) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
#[js::import = "None = $1"]
pub fn set_label(this: &ConsoleCounterError, val: &str) { panic!("web-sys intrinsic") }
impl ConsoleCounterError {
    #[doc = "Construct a new `ConsoleCounterError`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounterError`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
#[js::import = "None"]
pub fn error(&mut self, val: &str) -> &mut Self { self.set_error(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_label()` instead."]
#[js::import = "None"]
pub fn label(&mut self, val: &str) -> &mut Self { self.set_label(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConsoleCounterError {
    fn default() -> Self {
        Self::new()
    }
}
