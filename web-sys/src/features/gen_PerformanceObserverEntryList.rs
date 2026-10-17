#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceObserverEntryList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceObserverEntryList(pub ::wasm_bindgen::JsValue);
    #[doc = "The `getEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
impl PerformanceObserverEntryList {
    #[js::import = "$0.getEntries()"]
pub fn get_entries(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PerformanceEntryFilterOptions")]
    #[doc = "The `getEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceEntryFilterOptions`, `PerformanceObserverEntryList`*"]
impl PerformanceObserverEntryList {
    #[js::import = "$0.getEntries($1)"]
pub fn get_entries_with_filter(&self, filter: &PerformanceEntryFilterOptions) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getEntriesByName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
impl PerformanceObserverEntryList {
    #[js::import = "$0.getEntriesByName($1)"]
pub fn get_entries_by_name(&self, name: &str) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getEntriesByName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
impl PerformanceObserverEntryList {
    #[js::import = "$0.getEntriesByName($1, $2)"]
pub fn get_entries_by_name_with_entry_type(&self, name: &str, entry_type: &str) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getEntriesByType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverEntryList`*"]
impl PerformanceObserverEntryList {
    #[js::import = "$0.getEntriesByType($1)"]
pub fn get_entries_by_type(&self, entry_type: &str) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
