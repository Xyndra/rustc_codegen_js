#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PaintRequest` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintRequest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PaintRequest(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomRect")]
    #[doc = "Getter for the `clientRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/clientRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRect`, `PaintRequest`*"]
impl PaintRequest {
    #[js::import = "$0.clientRect"]
pub fn client_rect(&self) -> DomRect { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `reason` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PaintRequest/reason)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PaintRequest`*"]
impl PaintRequest {
    #[js::import = "$0.reason"]
pub fn reason(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
