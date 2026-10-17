#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceTiming` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceTiming(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `navigationStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/navigationStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.navigationStart"]
pub fn navigation_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `unloadEventStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.unloadEventStart"]
pub fn unload_event_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `unloadEventEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/unloadEventEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.unloadEventEnd"]
pub fn unload_event_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `redirectStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.redirectStart"]
pub fn redirect_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `redirectEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/redirectEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.redirectEnd"]
pub fn redirect_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fetchStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/fetchStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.fetchStart"]
pub fn fetch_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domainLookupStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.domainLookupStart"]
pub fn domain_lookup_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domainLookupEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domainLookupEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.domainLookupEnd"]
pub fn domain_lookup_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `connectStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.connectStart"]
pub fn connect_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `connectEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/connectEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.connectEnd"]
pub fn connect_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `secureConnectionStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/secureConnectionStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.secureConnectionStart"]
pub fn secure_connection_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `requestStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/requestStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.requestStart"]
pub fn request_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `responseStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.responseStart"]
pub fn response_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `responseEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/responseEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.responseEnd"]
pub fn response_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domLoading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domLoading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.domLoading"]
pub fn dom_loading(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domInteractive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domInteractive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.domInteractive"]
pub fn dom_interactive(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domContentLoadedEventStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.domContentLoadedEventStart"]
pub fn dom_content_loaded_event_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domContentLoadedEventEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domContentLoadedEventEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.domContentLoadedEventEnd"]
pub fn dom_content_loaded_event_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domComplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/domComplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.domComplete"]
pub fn dom_complete(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `loadEventStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.loadEventStart"]
pub fn load_event_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `loadEventEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/loadEventEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.loadEventEnd"]
pub fn load_event_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `timeToNonBlankPaint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToNonBlankPaint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.timeToNonBlankPaint"]
pub fn time_to_non_blank_paint(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `timeToDOMContentFlushed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/timeToDOMContentFlushed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.timeToDOMContentFlushed"]
pub fn time_to_dom_content_flushed(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceTiming/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceTiming`*"]
impl PerformanceTiming {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
