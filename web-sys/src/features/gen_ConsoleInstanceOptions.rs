#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleInstanceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConsoleInstanceOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None"]
pub fn get_console_id(this: &ConsoleInstanceOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `consoleID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None = $1"]
pub fn set_console_id(this: &ConsoleInstanceOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `dump` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None"]
pub fn get_dump(this: &ConsoleInstanceOptions) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `dump` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None = $1"]
pub fn set_dump(this: &ConsoleInstanceOptions, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None"]
pub fn get_inner_id(this: &ConsoleInstanceOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `innerID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None = $1"]
pub fn set_inner_id(this: &ConsoleInstanceOptions, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ConsoleLogLevel")]
    #[doc = "Get the `maxLogLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`, `ConsoleLogLevel`*"]
#[js::import = "None"]
pub fn get_max_log_level(this: &ConsoleInstanceOptions) -> Option<ConsoleLogLevel> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ConsoleLogLevel")]
    #[doc = "Change the `maxLogLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`, `ConsoleLogLevel`*"]
#[js::import = "None = $1"]
pub fn set_max_log_level(this: &ConsoleInstanceOptions, val: ConsoleLogLevel) { panic!("web-sys intrinsic") }
    #[doc = "Get the `maxLogLevelPref` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None"]
pub fn get_max_log_level_pref(this: &ConsoleInstanceOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `maxLogLevelPref` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None = $1"]
pub fn set_max_log_level_pref(this: &ConsoleInstanceOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None"]
pub fn get_prefix(this: &ConsoleInstanceOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None = $1"]
pub fn set_prefix(this: &ConsoleInstanceOptions, val: &str) { panic!("web-sys intrinsic") }
impl ConsoleInstanceOptions {
    #[doc = "Construct a new `ConsoleInstanceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleInstanceOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_console_id()` instead."]
#[js::import = "None"]
pub fn console_id(&mut self, val: &str) -> &mut Self { self.set_console_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_dump()` instead."]
#[js::import = "None"]
pub fn dump(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_dump(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_inner_id()` instead."]
#[js::import = "None"]
pub fn inner_id(&mut self, val: &str) -> &mut Self { self.set_inner_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ConsoleLogLevel")]
    #[deprecated = "Use `set_max_log_level()` instead."]
#[js::import = "None"]
pub fn max_log_level(&mut self, val: ConsoleLogLevel) -> &mut Self { self.set_max_log_level(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_max_log_level_pref()` instead."]
#[js::import = "None"]
pub fn max_log_level_pref(&mut self, val: &str) -> &mut Self { self.set_max_log_level_pref(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_prefix()` instead."]
#[js::import = "None"]
pub fn prefix(&mut self, val: &str) -> &mut Self { self.set_prefix(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConsoleInstanceOptions {
    fn default() -> Self {
        Self::new()
    }
}
