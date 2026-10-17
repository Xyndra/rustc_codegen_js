#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Performance` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Performance(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `timeOrigin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.timeOrigin"]
pub fn time_origin(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PerformanceTiming")]
    #[doc = "Getter for the `timing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`, `PerformanceTiming`*"]
impl Performance {
    #[js::import = "$0.timing"]
pub fn timing(&self) -> PerformanceTiming { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PerformanceNavigation")]
    #[doc = "Getter for the `navigation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/navigation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`, `PerformanceNavigation`*"]
impl Performance {
    #[js::import = "$0.navigation"]
pub fn navigation(&self) -> PerformanceNavigation { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onresourcetimingbufferfull` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/onresourcetimingbufferfull)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.onresourcetimingbufferfull"]
pub fn onresourcetimingbufferfull(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onresourcetimingbufferfull` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/onresourcetimingbufferfull)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.onresourcetimingbufferfull = $1"]
pub fn set_onresourcetimingbufferfull(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearMarks()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMarks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.clearMarks()"]
pub fn clear_marks(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearMarks()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMarks)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.clearMarks($1)"]
pub fn clear_marks_with_mark_name(&self, mark_name: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearMeasures()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMeasures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.clearMeasures()"]
pub fn clear_measures(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearMeasures()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearMeasures)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.clearMeasures($1)"]
pub fn clear_measures_with_measure_name(&self, measure_name: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearResourceTimings()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/clearResourceTimings)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.clearResourceTimings()"]
pub fn clear_resource_timings(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getEntries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.getEntries()"]
pub fn get_entries(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getEntriesByName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.getEntriesByName($1)"]
pub fn get_entries_by_name(&self, name: &str) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getEntriesByName()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.getEntriesByName($1, $2)"]
pub fn get_entries_by_name_with_entry_type(&self, name: &str, entry_type: &str) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `getEntriesByType()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/getEntriesByType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.getEntriesByType($1)"]
pub fn get_entries_by_type(&self, entry_type: &str) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `mark()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/mark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.mark($1)"]
pub fn mark(&self, mark_name: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `measure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.measure($1)"]
pub fn measure(&self, measure_name: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `measure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.measure($1, $2)"]
pub fn measure_with_start_mark(&self, measure_name: &str, start_mark: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `measure()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measure)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.measure($1, $2, $3)"]
pub fn measure_with_start_mark_and_end_mark(&self, measure_name: &str, start_mark: &str, end_mark: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `measureUserAgentSpecificMemory()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/measureUserAgentSpecificMemory)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Performance {
    #[js::import = "$0.measureUserAgentSpecificMemory()"]
pub fn measure_user_agent_specific_memory(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `now()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.now()"]
pub fn now(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `setResourceTimingBufferSize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/setResourceTimingBufferSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.setResourceTimingBufferSize($1)"]
pub fn set_resource_timing_buffer_size(&self, max_size: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Performance/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`*"]
impl Performance {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
