#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialPropertiesOutput` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CredentialPropertiesOutput(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `rk` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
#[js::import = "None"]
pub fn get_rk(this: &CredentialPropertiesOutput) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rk` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
#[js::import = "None = $1"]
pub fn set_rk(this: &CredentialPropertiesOutput, val: bool) { panic!("web-sys intrinsic") }
impl CredentialPropertiesOutput {
    #[doc = "Construct a new `CredentialPropertiesOutput`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialPropertiesOutput`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_rk()` instead."]
#[js::import = "None"]
pub fn rk(&mut self, val: bool) -> &mut Self { self.set_rk(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for CredentialPropertiesOutput {
    fn default() -> Self {
        Self::new()
    }
}
