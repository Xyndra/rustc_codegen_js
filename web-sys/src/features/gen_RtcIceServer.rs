#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceServer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcIceServer(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `credential` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::import = "None"]
pub fn get_credential(this: &RtcIceServer) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `credential` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::import = "None = $1"]
pub fn set_credential(this: &RtcIceServer, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcIceCredentialType")]
    #[doc = "Get the `credentialType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCredentialType`, `RtcIceServer`*"]
#[js::import = "None"]
pub fn get_credential_type(this: &RtcIceServer) -> Option<RtcIceCredentialType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RtcIceCredentialType")]
    #[doc = "Change the `credentialType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceCredentialType`, `RtcIceServer`*"]
#[js::import = "None = $1"]
pub fn set_credential_type(this: &RtcIceServer, val: RtcIceCredentialType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::import = "None"]
pub fn get_url(this: &RtcIceServer) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::import = "None = $1"]
pub fn set_url(this: &RtcIceServer, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `urls` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::import = "None"]
pub fn get_urls(this: &RtcIceServer) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `urls` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::import = "None = $1"]
pub fn set_urls(this: &RtcIceServer, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `username` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::import = "None"]
pub fn get_username(this: &RtcIceServer) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `username` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::import = "None = $1"]
pub fn set_username(this: &RtcIceServer, val: &str) { panic!("web-sys intrinsic") }
impl RtcIceServer {
    #[doc = "Construct a new `RtcIceServer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceServer`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_credential()` instead."]
#[js::import = "None"]
pub fn credential(&mut self, val: &str) -> &mut Self { self.set_credential(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "RtcIceCredentialType")]
    #[deprecated = "Use `set_credential_type()` instead."]
#[js::import = "None"]
pub fn credential_type(&mut self, val: RtcIceCredentialType) -> &mut Self { self.set_credential_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_url()` instead."]
#[js::import = "None"]
pub fn url(&mut self, val: &str) -> &mut Self { self.set_url(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_urls()` instead."]
#[js::import = "None"]
pub fn urls(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_urls(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_username()` instead."]
#[js::import = "None"]
pub fn username(&mut self, val: &str) -> &mut Self { self.set_username(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcIceServer {
    fn default() -> Self {
        Self::new()
    }
}
