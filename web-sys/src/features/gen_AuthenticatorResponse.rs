#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorResponse` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorResponse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorResponse`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AuthenticatorResponse(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `clientDataJSON` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/AuthenticatorResponse/clientDataJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorResponse`*"]
impl AuthenticatorResponse {
    #[js::import = "$0.clientDataJSON"]
pub fn client_data_json(&self) -> ::js_sys::ArrayBuffer { panic!("web-sys intrinsic") }
}
