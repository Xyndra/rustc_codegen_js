#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceEntryFilterOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceEntryFilterOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
#[js::import = "None"]
pub fn get_entry_type(this: &PerformanceEntryFilterOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_entry_type(this: &PerformanceEntryFilterOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `initiatorType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
#[js::import = "None"]
pub fn get_initiator_type(this: &PerformanceEntryFilterOptions,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `initiatorType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_initiator_type(this: &PerformanceEntryFilterOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
#[js::import = "None"]
pub fn get_name(this: &PerformanceEntryFilterOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &PerformanceEntryFilterOptions, val: &str) { panic!("web-sys intrinsic") }
impl PerformanceEntryFilterOptions {
    #[doc = "Construct a new `PerformanceEntryFilterOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_entry_type()` instead."]
#[js::import = "None"]
pub fn entry_type(&mut self, val: &str) -> &mut Self { self.set_entry_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_initiator_type()` instead."]
#[js::import = "None"]
pub fn initiator_type(&mut self, val: &str) -> &mut Self { self.set_initiator_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PerformanceEntryFilterOptions {
    fn default() -> Self {
        Self::new()
    }
}
