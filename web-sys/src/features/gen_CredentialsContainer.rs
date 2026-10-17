#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialsContainer` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CredentialsContainer(pub ::wasm_bindgen::JsValue);
    #[doc = "The `create()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
impl CredentialsContainer {
    #[js::import = "$0.create()"]
pub fn create(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CredentialCreationOptions")]
    #[doc = "The `create()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/create)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`, `CredentialsContainer`*"]
impl CredentialsContainer {
    #[js::import = "$0.create($1)"]
pub fn create_with_options(&self, options: &CredentialCreationOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
impl CredentialsContainer {
    #[js::import = "$0.get()"]
pub fn get(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CredentialRequestOptions")]
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialRequestOptions`, `CredentialsContainer`*"]
impl CredentialsContainer {
    #[js::import = "$0.get($1)"]
pub fn get_with_options(&self, options: &CredentialRequestOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `preventSilentAccess()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/preventSilentAccess)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialsContainer`*"]
impl CredentialsContainer {
    #[js::import = "$0.preventSilentAccess()"]
pub fn prevent_silent_access(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Credential")]
    #[doc = "The `store()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CredentialsContainer/store)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Credential`, `CredentialsContainer`*"]
impl CredentialsContainer {
    #[js::import = "$0.store($1)"]
pub fn store(&self, credential: &Credential) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
