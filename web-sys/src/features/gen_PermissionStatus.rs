#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PermissionStatus` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionStatus`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PermissionStatus(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PermissionState")]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionState`, `PermissionStatus`*"]
impl PermissionStatus {
    #[js::import = "$0.state"]
pub fn state(&self) -> PermissionState { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionStatus`*"]
impl PermissionStatus {
    #[js::import = "$0.onchange"]
pub fn onchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionStatus`*"]
impl PermissionStatus {
    #[js::import = "$0.onchange = $1"]
pub fn set_onchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
