#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UserActivation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserActivation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserActivation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct UserActivation(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `hasBeenActive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserActivation/hasBeenActive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserActivation`*"]
impl UserActivation {
    #[js::import = "$0.hasBeenActive"]
pub fn has_been_active(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isActive` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UserActivation/isActive)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserActivation`*"]
impl UserActivation {
    #[js::import = "$0.isActive"]
pub fn is_active(&self) -> bool { panic!("web-sys intrinsic") }
}
