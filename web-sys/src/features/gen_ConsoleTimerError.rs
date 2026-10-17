#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleTimerError` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConsoleTimerError(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
#[js::import = "None"]
pub fn get_error(this: &ConsoleTimerError) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
#[js::import = "None = $1"]
pub fn set_error(this: &ConsoleTimerError, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
#[js::import = "None"]
pub fn get_name(this: &ConsoleTimerError) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &ConsoleTimerError, val: &str) { panic!("web-sys intrinsic") }
impl ConsoleTimerError {
    #[doc = "Construct a new `ConsoleTimerError`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerError`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_error()` instead."]
#[js::import = "None"]
pub fn error(&mut self, val: &str) -> &mut Self { self.set_error(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConsoleTimerError {
    fn default() -> Self {
        Self::new()
    }
}
