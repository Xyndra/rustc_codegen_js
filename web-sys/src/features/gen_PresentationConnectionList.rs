#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationConnectionList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PresentationConnectionList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `connections` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/connections)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
impl PresentationConnectionList {
    #[js::import = "$0.connections"]
pub fn connections(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
impl PresentationConnectionList {
    #[js::import = "$0.onconnectionavailable"]
pub fn onconnectionavailable(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onconnectionavailable` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationConnectionList/onconnectionavailable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationConnectionList`*"]
impl PresentationConnectionList {
    #[js::import = "$0.onconnectionavailable = $1"]
pub fn set_onconnectionavailable(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
