#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorkerGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ServiceWorkerGlobalScope(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Clients")]
    #[doc = "Getter for the `clients` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/clients)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clients`, `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.clients"]
pub fn clients(&self) -> Clients { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ServiceWorkerRegistration")]
    #[doc = "Getter for the `registration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/registration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`, `ServiceWorkerRegistration`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.registration"]
pub fn registration(&self) -> ServiceWorkerRegistration { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oninstall` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.oninstall"]
pub fn oninstall(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oninstall` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/oninstall)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.oninstall = $1"]
pub fn set_oninstall(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onactivate"]
pub fn onactivate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onactivate = $1"]
pub fn set_onactivate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onfetch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onfetch"]
pub fn onfetch(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onfetch` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onfetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onfetch = $1"]
pub fn set_onfetch(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpush` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onpush"]
pub fn onpush(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpush` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onpush = $1"]
pub fn set_onpush(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpushsubscriptionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onpushsubscriptionchange"]
pub fn onpushsubscriptionchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpushsubscriptionchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onpushsubscriptionchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onpushsubscriptionchange = $1"]
pub fn set_onpushsubscriptionchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onnotificationclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onnotificationclick"]
pub fn onnotificationclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onnotificationclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onnotificationclick = $1"]
pub fn set_onnotificationclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onnotificationclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onnotificationclose"]
pub fn onnotificationclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onnotificationclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/onnotificationclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.onnotificationclose = $1"]
pub fn set_onnotificationclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `skipWaiting()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerGlobalScope/skipWaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerGlobalScope`*"]
impl ServiceWorkerGlobalScope {
    #[js::import = "$0.skipWaiting()"]
pub fn skip_waiting(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
