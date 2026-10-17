#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ByteLengthQueuingStrategy` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ByteLengthQueuingStrategy`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ByteLengthQueuingStrategy(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy/highWaterMark)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ByteLengthQueuingStrategy`*"]
impl ByteLengthQueuingStrategy {
    #[js::import = "$0.highWaterMark"]
pub fn high_water_mark(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ByteLengthQueuingStrategy`*"]
impl ByteLengthQueuingStrategy {
    #[js::import = "$0.size"]
pub fn size(&self) -> ::js_sys::Function { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "QueuingStrategyInit")]
    #[doc = "The `new ByteLengthQueuingStrategy(..)` constructor, creating a new instance of `ByteLengthQueuingStrategy`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy/ByteLengthQueuingStrategy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ByteLengthQueuingStrategy`, `QueuingStrategyInit`*"]
impl ByteLengthQueuingStrategy {
    #[js::import = "new ByteLengthQueuingStrategy($0)"]
pub fn new(init: &QueuingStrategyInit) -> Result<ByteLengthQueuingStrategy, JsValue> { panic!("web-sys intrinsic") }
}
