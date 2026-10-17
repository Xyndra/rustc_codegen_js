#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceObserver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceObserver(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new PerformanceObserver(..)` constructor, creating a new instance of `PerformanceObserver`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/PerformanceObserver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`*"]
impl PerformanceObserver {
    #[js::import = "new PerformanceObserver($0)"]
pub fn new(callback: &::js_sys::Function) -> Result<PerformanceObserver, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `disconnect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/disconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`*"]
impl PerformanceObserver {
    #[js::import = "$0.disconnect()"]
pub fn disconnect(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PerformanceObserverInit")]
    #[doc = "The `observe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/observe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`, `PerformanceObserverInit`*"]
impl PerformanceObserver {
    #[js::import = "$0.observe($1)"]
pub fn observe(&self, options: &PerformanceObserverInit) { panic!("web-sys intrinsic") }
}
    #[doc = "The `takeRecords()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/takeRecords)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserver`*"]
impl PerformanceObserver {
    #[js::import = "$0.takeRecords()"]
pub fn take_records(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
