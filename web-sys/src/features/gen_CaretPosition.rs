#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CaretPosition` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CaretPosition(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `offsetNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offsetNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`, `Node`*"]
impl CaretPosition {
    #[js::import = "$0.offsetNode"]
pub fn offset_node(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `offset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/offset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`*"]
impl CaretPosition {
    #[js::import = "$0.offset"]
pub fn offset(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRect")]
    #[doc = "The `getClientRect()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CaretPosition/getClientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CaretPosition`, `DomRect`*"]
impl CaretPosition {
    #[js::import = "$0.getClientRect()"]
pub fn get_client_rect(&self) -> Option<DomRect> { panic!("web-sys intrinsic") }
}
