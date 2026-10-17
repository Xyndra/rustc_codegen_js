#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServiceWorkerContainer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ServiceWorkerContainer(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ServiceWorker")]
    #[doc = "Getter for the `controller` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/controller)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorker`, `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.controller"]
pub fn controller(&self) -> Option<ServiceWorker> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.ready"]
pub fn ready(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncontrollerchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.oncontrollerchange"]
pub fn oncontrollerchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncontrollerchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/oncontrollerchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.oncontrollerchange = $1"]
pub fn set_oncontrollerchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRegistration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.getRegistration()"]
pub fn get_registration(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRegistration()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.getRegistration($1)"]
pub fn get_registration_with_document_url(&self, document_url: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRegistrations()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getRegistrations)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.getRegistrations()"]
pub fn get_registrations(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `getScopeForUrl()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/getScopeForUrl)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.getScopeForUrl($1)"]
pub fn get_scope_for_url(&self, url: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `register()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.register($1)"]
pub fn register(&self, script_url: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RegistrationOptions")]
    #[doc = "The `register()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorkerContainer/register)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerContainer`*"]
impl ServiceWorkerContainer {
    #[js::import = "$0.register($1, $2)"]
pub fn register_with_options(&self, script_url: &str, options: &RegistrationOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
