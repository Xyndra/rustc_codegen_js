#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BarProp` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BarProp(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `visible` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`*"]
impl BarProp {
    #[js::import = "$0.visible"]
pub fn visible(&self) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `visible` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`*"]
impl BarProp {
    #[js::import = "$0.visible = $1"]
pub fn set_visible(&self, value: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
