#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceEntry` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceEntry(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
impl PerformanceEntry {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `entryType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/entryType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
impl PerformanceEntry {
    #[js::import = "$0.entryType"]
pub fn entry_type(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `startTime` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/startTime)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
impl PerformanceEntry {
    #[js::import = "$0.startTime"]
pub fn start_time(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
impl PerformanceEntry {
    #[js::import = "$0.duration"]
pub fn duration(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntry`*"]
impl PerformanceEntry {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
