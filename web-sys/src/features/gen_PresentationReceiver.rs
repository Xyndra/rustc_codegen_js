#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PresentationReceiver` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationReceiver`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PresentationReceiver(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `connectionList` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PresentationReceiver/connectionList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PresentationReceiver`*"]
impl PresentationReceiver {
    #[js::import = "$0.connectionList"]
pub fn connection_list(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
