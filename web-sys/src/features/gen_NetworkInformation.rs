#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NetworkInformation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkInformation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NetworkInformation(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ConnectionType")]
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnectionType`, `NetworkInformation`*"]
impl NetworkInformation {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ConnectionType { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontypechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkInformation`*"]
impl NetworkInformation {
    #[js::import = "$0.ontypechange"]
pub fn ontypechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontypechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/ontypechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkInformation`*"]
impl NetworkInformation {
    #[js::import = "$0.ontypechange = $1"]
pub fn set_ontypechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
