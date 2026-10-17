#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TimeRanges` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeRanges`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TimeRanges(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeRanges`*"]
impl TimeRanges {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `end()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/end)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeRanges`*"]
impl TimeRanges {
    #[js::import = "$0.end($1)"]
pub fn end(&self, index: u32) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `start()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/TimeRanges/start)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TimeRanges`*"]
impl TimeRanges {
    #[js::import = "$0.start($1)"]
pub fn start(&self, index: u32) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
