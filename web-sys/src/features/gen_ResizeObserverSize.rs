#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResizeObserverSize` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverSize`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ResizeObserverSize(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `inlineSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverSize/inlineSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverSize`*"]
impl ResizeObserverSize {
    #[js::import = "$0.inlineSize"]
pub fn inline_size(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `blockSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserverSize/blockSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResizeObserverSize`*"]
impl ResizeObserverSize {
    #[js::import = "$0.blockSize"]
pub fn block_size(&self) -> f64 { panic!("web-sys intrinsic") }
}
