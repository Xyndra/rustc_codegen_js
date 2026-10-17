#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleStackEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConsoleStackEntry(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None"]
pub fn get_async_cause(this: &ConsoleStackEntry) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None = $1"]
pub fn set_async_cause(this: &ConsoleStackEntry, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None"]
pub fn get_column_number(this: &ConsoleStackEntry) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None = $1"]
pub fn set_column_number(this: &ConsoleStackEntry, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None"]
pub fn get_filename(this: &ConsoleStackEntry) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None = $1"]
pub fn set_filename(this: &ConsoleStackEntry, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None"]
pub fn get_function_name(this: &ConsoleStackEntry) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `functionName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None = $1"]
pub fn set_function_name(this: &ConsoleStackEntry, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None"]
pub fn get_line_number(this: &ConsoleStackEntry) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None = $1"]
pub fn set_line_number(this: &ConsoleStackEntry, val: u32) { panic!("web-sys intrinsic") }
impl ConsoleStackEntry {
    #[doc = "Construct a new `ConsoleStackEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleStackEntry`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_async_cause()` instead."]
#[js::import = "None"]
pub fn async_cause(&mut self, val: Option<&str>) -> &mut Self { self.set_async_cause(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_column_number()` instead."]
#[js::import = "None"]
pub fn column_number(&mut self, val: u32) -> &mut Self { self.set_column_number(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_filename()` instead."]
#[js::import = "None"]
pub fn filename(&mut self, val: &str) -> &mut Self { self.set_filename(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_function_name()` instead."]
#[js::import = "None"]
pub fn function_name(&mut self, val: &str) -> &mut Self { self.set_function_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_line_number()` instead."]
#[js::import = "None"]
pub fn line_number(&mut self, val: u32) -> &mut Self { self.set_line_number(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConsoleStackEntry {
    fn default() -> Self {
        Self::new()
    }
}
