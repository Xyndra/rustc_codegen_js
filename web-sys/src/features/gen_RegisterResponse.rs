#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegisterResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RegisterResponse(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None"]
pub fn get_client_data(this: &RegisterResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None = $1"]
pub fn set_client_data(this: &RegisterResponse, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None"]
pub fn get_error_code(this: &RegisterResponse) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None = $1"]
pub fn set_error_code(this: &RegisterResponse, val: Option<u16>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None"]
pub fn get_error_message(this: &RegisterResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None = $1"]
pub fn set_error_message(this: &RegisterResponse, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `registrationData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None"]
pub fn get_registration_data(this: &RegisterResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `registrationData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None = $1"]
pub fn set_registration_data(this: &RegisterResponse, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None"]
pub fn get_version(this: &RegisterResponse) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
#[js::import = "None = $1"]
pub fn set_version(this: &RegisterResponse, val: &str) { panic!("web-sys intrinsic") }
impl RegisterResponse {
    #[doc = "Construct a new `RegisterResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
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
    #[deprecated = "Use `set_registration_data()` instead."]
#[js::import = "None"]
pub fn registration_data(&mut self, val: &str) -> &mut Self { self.set_registration_data(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_version()` instead."]
#[js::import = "None"]
pub fn version(&mut self, val: &str) -> &mut Self { self.set_version(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RegisterResponse {
    fn default() -> Self {
        Self::new()
    }
}
