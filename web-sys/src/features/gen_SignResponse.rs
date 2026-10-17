#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SignResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SignResponse(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None"]
pub fn get_client_data(this: &SignResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None = $1"]
pub fn set_client_data(this: &SignResponse, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None"]
pub fn get_error_code(this: &SignResponse) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None = $1"]
pub fn set_error_code(this: &SignResponse, val: Option<u16>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None"]
pub fn get_error_message(this: &SignResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None = $1"]
pub fn set_error_message(this: &SignResponse, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None"]
pub fn get_key_handle(this: &SignResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `keyHandle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None = $1"]
pub fn set_key_handle(this: &SignResponse, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `signatureData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None"]
pub fn get_signature_data(this: &SignResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `signatureData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None = $1"]
pub fn set_signature_data(this: &SignResponse, val: &str) { panic!("web-sys intrinsic") }
impl SignResponse {
    #[doc = "Construct a new `SignResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SignResponse`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_client_data()` instead."]
#[js::import = "None"]
pub fn client_data(&mut self, val: &str) -> &mut Self { self.set_client_data(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_error_code()` instead."]
#[js::import = "None"]
pub fn error_code(&mut self, val: Option<u16>) -> &mut Self { self.set_error_code(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_error_message()` instead."]
#[js::import = "None"]
pub fn error_message(&mut self, val: Option<&str>) -> &mut Self { self.set_error_message(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_key_handle()` instead."]
#[js::import = "None"]
pub fn key_handle(&mut self, val: &str) -> &mut Self { self.set_key_handle(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_signature_data()` instead."]
#[js::import = "None"]
pub fn signature_data(&mut self, val: &str) -> &mut Self { self.set_signature_data(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for SignResponse {
    fn default() -> Self {
        Self::new()
    }
}
