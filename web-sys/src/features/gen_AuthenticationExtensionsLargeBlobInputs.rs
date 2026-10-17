#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsLargeBlobInputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AuthenticationExtensionsLargeBlobInputs(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `read` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::import = "None"]
pub fn get_read(this: &AuthenticationExtensionsLargeBlobInputs) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `read` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::import = "None = $1"]
pub fn set_read(this: &AuthenticationExtensionsLargeBlobInputs, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `support` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::import = "None"]
pub fn get_support(this: &AuthenticationExtensionsLargeBlobInputs,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `support` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::import = "None = $1"]
pub fn set_support(this: &AuthenticationExtensionsLargeBlobInputs, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::import = "None"]
pub fn get_write(this: &AuthenticationExtensionsLargeBlobInputs) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::import = "None = $1"]
pub fn set_write(this: &AuthenticationExtensionsLargeBlobInputs, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
impl AuthenticationExtensionsLargeBlobInputs {
    #[doc = "Construct a new `AuthenticationExtensionsLargeBlobInputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobInputs`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_read()` instead."]
#[js::import = "None"]
pub fn read(&mut self, val: bool) -> &mut Self { self.set_read(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_support()` instead."]
#[js::import = "None"]
pub fn support(&mut self, val: &str) -> &mut Self { self.set_support(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_write()` instead."]
#[js::import = "None"]
pub fn write(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_write(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AuthenticationExtensionsLargeBlobInputs {
    fn default() -> Self {
        Self::new()
    }
}
