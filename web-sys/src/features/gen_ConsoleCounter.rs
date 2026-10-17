#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleCounter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConsoleCounter(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `count` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
#[js::import = "None"]
pub fn get_count(this: &ConsoleCounter) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `count` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
#[js::import = "None = $1"]
pub fn set_count(this: &ConsoleCounter, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
#[js::import = "None"]
pub fn get_label(this: &ConsoleCounter) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
#[js::import = "None = $1"]
pub fn set_label(this: &ConsoleCounter, val: &str) { panic!("web-sys intrinsic") }
impl ConsoleCounter {
    #[doc = "Construct a new `ConsoleCounter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleCounter`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_count()` instead."]
#[js::import = "None"]
pub fn count(&mut self, val: u32) -> &mut Self { self.set_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_label()` instead."]
#[js::import = "None"]
pub fn label(&mut self, val: &str) -> &mut Self { self.set_label(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConsoleCounter {
    fn default() -> Self {
        Self::new()
    }
}
