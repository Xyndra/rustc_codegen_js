#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorkerRegistration` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ServiceWorkerRegistration(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ServiceWorker")]
    #[doc = "Getter for the `installing` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/installing)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.installing"]
pub fn installing(&self) -> Option<ServiceWorker> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServiceWorker")]
    #[doc = "Getter for the `waiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/waiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.waiting"]
pub fn waiting(&self) -> Option<ServiceWorker> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServiceWorker")]
    #[doc = "Getter for the `active` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/active)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.active"]
pub fn active(&self) -> Option<ServiceWorker> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scope` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/scope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.scope"]
pub fn scope(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Getter for the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/updateViaCache)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`, `ServiceWorkerUpdateViaCache`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.updateViaCache"]
pub fn update_via_cache(&self) -> Result<ServiceWorkerUpdateViaCache, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onupdatefound` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.onupdatefound"]
pub fn onupdatefound(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onupdatefound` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/onupdatefound)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.onupdatefound = $1"]
pub fn set_onupdatefound(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "PushManager")]
    #[doc = "Getter for the `pushManager` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/pushManager)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushManager`, `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.pushManager"]
pub fn push_manager(&self) -> Result<PushManager, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getNotifications()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/getNotifications)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.getNotifications()"]
pub fn get_notifications(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `showNotification()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.showNotification($1)"]
pub fn show_notification(&self, title: &str) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "NotificationOptions")]
    #[doc = "The `showNotification()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/showNotification)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NotificationOptions`, `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.showNotification($1, $2)"]
pub fn show_notification_with_options(&self, title: &str, options: &NotificationOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `unregister()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/unregister)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.unregister()"]
pub fn unregister(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `update()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerRegistration/update)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerRegistration`*"]
impl ServiceWorkerRegistration {
    #[js::import = "$0.update()"]
pub fn update(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
