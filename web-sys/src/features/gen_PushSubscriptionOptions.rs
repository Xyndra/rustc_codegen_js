#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionOptions` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushSubscriptionOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushSubscriptionOptions/applicationServerKey)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptions`*"]
impl PushSubscriptionOptions {
    #[js::import = "$0.applicationServerKey"]
pub fn application_server_key(&self) -> Result<Option<::js_sys::ArrayBuffer>, JsValue> { panic!("web-sys intrinsic") }
}
