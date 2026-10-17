#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushManager` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushManager(pub ::wasm_bindgen::JsValue);
    #[doc = "The `getSubscription()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/getSubscription)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`*"]
impl PushManager {
    #[js::import = "$0.getSubscription()"]
pub fn get_subscription(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `permissionState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`*"]
impl PushManager {
    #[js::import = "$0.permissionState()"]
pub fn permission_state(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PushSubscriptionOptionsInit")]
    #[doc = "The `permissionState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/permissionState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`, `PushSubscriptionOptionsInit`*"]
impl PushManager {
    #[js::import = "$0.permissionState($1)"]
pub fn permission_state_with_options(&self, options: &PushSubscriptionOptionsInit) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `subscribe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`*"]
impl PushManager {
    #[js::import = "$0.subscribe()"]
pub fn subscribe(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PushSubscriptionOptionsInit")]
    #[doc = "The `subscribe()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PushManager/subscribe)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`, `PushSubscriptionOptionsInit`*"]
impl PushManager {
    #[js::import = "$0.subscribe($1)"]
pub fn subscribe_with_options(&self, options: &PushSubscriptionOptionsInit) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
