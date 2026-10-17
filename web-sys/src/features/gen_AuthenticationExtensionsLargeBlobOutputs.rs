#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsLargeBlobOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AuthenticationExtensionsLargeBlobOutputs(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `blob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::import = "None"]
pub fn get_blob(this: &AuthenticationExtensionsLargeBlobOutputs,) -> Option<::js_sys::ArrayBuffer> { panic!("web-sys intrinsic") }
    #[doc = "Change the `blob` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::import = "None = $1"]
pub fn set_blob(this: &AuthenticationExtensionsLargeBlobOutputs, val: &::js_sys::ArrayBuffer) { panic!("web-sys intrinsic") }
    #[doc = "Get the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::import = "None"]
pub fn get_supported(this: &AuthenticationExtensionsLargeBlobOutputs) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::import = "None = $1"]
pub fn set_supported(this: &AuthenticationExtensionsLargeBlobOutputs, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `written` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::import = "None"]
pub fn get_written(this: &AuthenticationExtensionsLargeBlobOutputs) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `written` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::import = "None = $1"]
pub fn set_written(this: &AuthenticationExtensionsLargeBlobOutputs, val: bool) { panic!("web-sys intrinsic") }
impl AuthenticationExtensionsLargeBlobOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsLargeBlobOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsLargeBlobOutputs`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_blob()` instead."]
#[js::import = "None"]
pub fn blob(&mut self, val: &::js_sys::ArrayBuffer) -> &mut Self { self.set_blob(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_supported()` instead."]
#[js::import = "None"]
pub fn supported(&mut self, val: bool) -> &mut Self { self.set_supported(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_written()` instead."]
#[js::import = "None"]
pub fn written(&mut self, val: bool) -> &mut Self { self.set_written(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AuthenticationExtensionsLargeBlobOutputs {
    fn default() -> Self {
        Self::new()
    }
}
