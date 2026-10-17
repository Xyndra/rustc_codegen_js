#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleProfileEvent` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConsoleProfileEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
#[js::import = "None"]
pub fn get_action(this: &ConsoleProfileEvent) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
#[js::import = "None = $1"]
pub fn set_action(this: &ConsoleProfileEvent, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
#[js::import = "None"]
pub fn get_arguments(this: &ConsoleProfileEvent) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
#[js::import = "None = $1"]
pub fn set_arguments(this: &ConsoleProfileEvent, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl ConsoleProfileEvent {
    #[doc = "Construct a new `ConsoleProfileEvent`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_action()` instead."]
#[js::import = "None"]
pub fn action(&mut self, val: &str) -> &mut Self { self.set_action(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_arguments()` instead."]
#[js::import = "None"]
pub fn arguments(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_arguments(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConsoleProfileEvent {
    fn default() -> Self {
        Self::new()
    }
}
