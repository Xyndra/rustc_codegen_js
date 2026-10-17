#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CountQueuingStrategy` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CountQueuingStrategy`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CountQueuingStrategy(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy/highWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CountQueuingStrategy`*"]
impl CountQueuingStrategy {
    #[js::import = "$0.highWaterMark"]
pub fn high_water_mark(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CountQueuingStrategy`*"]
impl CountQueuingStrategy {
    #[js::import = "$0.size"]
pub fn size(&self) -> ::js_sys::Function { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "QueuingStrategyInit")]
    #[doc = "The `new CountQueuingStrategy(..)` constructor, creating a new instance of `CountQueuingStrategy`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy/CountQueuingStrategy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CountQueuingStrategy`, `QueuingStrategyInit`*"]
impl CountQueuingStrategy {
    #[js::import = "new CountQueuingStrategy($0)"]
pub fn new(init: &QueuingStrategyInit) -> Result<CountQueuingStrategy, JsValue> { panic!("web-sys intrinsic") }
}
