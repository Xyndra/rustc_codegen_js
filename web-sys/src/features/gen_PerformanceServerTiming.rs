#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceServerTiming` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceServerTiming(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
impl PerformanceServerTiming {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
impl PerformanceServerTiming {
    #[js::import = "$0.duration"]
pub fn duration(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `description` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/description)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
impl PerformanceServerTiming {
    #[js::import = "$0.description"]
pub fn description(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceServerTiming/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceServerTiming`*"]
impl PerformanceServerTiming {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
