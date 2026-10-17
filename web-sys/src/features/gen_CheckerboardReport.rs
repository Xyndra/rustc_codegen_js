#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CheckerboardReport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CheckerboardReport(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `log` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
#[js::import = "None"]
pub fn get_log(this: &CheckerboardReport) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `log` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
#[js::import = "None = $1"]
pub fn set_log(this: &CheckerboardReport, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "CheckerboardReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReason`, `CheckerboardReport`*"]
#[js::import = "None"]
pub fn get_reason(this: &CheckerboardReport) -> Option<CheckerboardReason> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CheckerboardReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReason`, `CheckerboardReport`*"]
#[js::import = "None = $1"]
pub fn set_reason(this: &CheckerboardReport, val: CheckerboardReason) { panic!("web-sys intrinsic") }
    #[doc = "Get the `severity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
#[js::import = "None"]
pub fn get_severity(this: &CheckerboardReport) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `severity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
#[js::import = "None = $1"]
pub fn set_severity(this: &CheckerboardReport, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
#[js::import = "None"]
pub fn get_timestamp(this: &CheckerboardReport) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
#[js::import = "None = $1"]
pub fn set_timestamp(this: &CheckerboardReport, val: f64) { panic!("web-sys intrinsic") }
impl CheckerboardReport {
    #[doc = "Construct a new `CheckerboardReport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_log()` instead."]
#[js::import = "None"]
pub fn log(&mut self, val: &str) -> &mut Self { self.set_log(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "CheckerboardReason")]
    #[deprecated = "Use `set_reason()` instead."]
#[js::import = "None"]
pub fn reason(&mut self, val: CheckerboardReason) -> &mut Self { self.set_reason(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_severity()` instead."]
#[js::import = "None"]
pub fn severity(&mut self, val: u32) -> &mut Self { self.set_severity(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_timestamp()` instead."]
#[js::import = "None"]
pub fn timestamp(&mut self, val: f64) -> &mut Self { self.set_timestamp(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for CheckerboardReport {
    fn default() -> Self {
        Self::new()
    }
}
