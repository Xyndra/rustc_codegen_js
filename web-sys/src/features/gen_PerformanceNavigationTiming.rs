#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceNavigationTiming` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceNavigationTiming(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `unloadEventStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.unloadEventStart"]
pub fn unload_event_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `unloadEventEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/unloadEventEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.unloadEventEnd"]
pub fn unload_event_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domInteractive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domInteractive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.domInteractive"]
pub fn dom_interactive(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domContentLoadedEventStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.domContentLoadedEventStart"]
pub fn dom_content_loaded_event_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domContentLoadedEventEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domContentLoadedEventEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.domContentLoadedEventEnd"]
pub fn dom_content_loaded_event_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domComplete` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/domComplete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.domComplete"]
pub fn dom_complete(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `loadEventStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.loadEventStart"]
pub fn load_event_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `loadEventEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/loadEventEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.loadEventEnd"]
pub fn load_event_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NavigationType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigationType`, `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.type"]
pub fn type_(&self) -> NavigationType { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `redirectCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/redirectCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.redirectCount"]
pub fn redirect_count(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigationTiming/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigationTiming`*"]
impl PerformanceNavigationTiming {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
