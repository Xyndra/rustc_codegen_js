#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorSelectionCriteria` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AuthenticatorSelectionCriteria(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "AuthenticatorAttachment")]
    #[doc = "Get the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttachment`, `AuthenticatorSelectionCriteria`*"]
#[js::import = "None"]
pub fn get_authenticator_attachment(this: &AuthenticatorSelectionCriteria,) -> Option<AuthenticatorAttachment> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AuthenticatorAttachment")]
    #[doc = "Change the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttachment`, `AuthenticatorSelectionCriteria`*"]
#[js::import = "None = $1"]
pub fn set_authenticator_attachment(this: &AuthenticatorSelectionCriteria, val: AuthenticatorAttachment,) { panic!("web-sys intrinsic") }
    #[doc = "Get the `requireResidentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
#[js::import = "None"]
pub fn get_require_resident_key(this: &AuthenticatorSelectionCriteria) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `requireResidentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
#[js::import = "None = $1"]
pub fn set_require_resident_key(this: &AuthenticatorSelectionCriteria, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `residentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
#[js::import = "None"]
pub fn get_resident_key(this: &AuthenticatorSelectionCriteria,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `residentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
#[js::import = "None = $1"]
pub fn set_resident_key(this: &AuthenticatorSelectionCriteria, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Get the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `UserVerificationRequirement`*"]
#[js::import = "None"]
pub fn get_user_verification(this: &AuthenticatorSelectionCriteria,) -> Option<UserVerificationRequirement> { panic!("web-sys intrinsic") }
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Change the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `UserVerificationRequirement`*"]
#[js::import = "None = $1"]
pub fn set_user_verification(this: &AuthenticatorSelectionCriteria, val: UserVerificationRequirement,) { panic!("web-sys intrinsic") }
impl AuthenticatorSelectionCriteria {
    #[doc = "Construct a new `AuthenticatorSelectionCriteria`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "AuthenticatorAttachment")]
    #[deprecated = "Use `set_authenticator_attachment()` instead."]
#[js::import = "None"]
pub fn authenticator_attachment(&mut self, val: AuthenticatorAttachment) -> &mut Self { self.set_authenticator_attachment(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_require_resident_key()` instead."]
#[js::import = "None"]
pub fn require_resident_key(&mut self, val: bool) -> &mut Self { self.set_require_resident_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_resident_key()` instead."]
#[js::import = "None"]
pub fn resident_key(&mut self, val: &str) -> &mut Self { self.set_resident_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "UserVerificationRequirement")]
    #[deprecated = "Use `set_user_verification()` instead."]
#[js::import = "None"]
pub fn user_verification(&mut self, val: UserVerificationRequirement) -> &mut Self { self.set_user_verification(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AuthenticatorSelectionCriteria {
    fn default() -> Self {
        Self::new()
    }
}
