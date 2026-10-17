#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PublicKeyCredentialParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`*"]
#[js::import = "None"]
pub fn get_alg(this: &PublicKeyCredentialParameters) -> i32 { panic!("web-sys intrinsic") }
    #[doc = "Change the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`*"]
#[js::import = "None = $1"]
pub fn set_alg(this: &PublicKeyCredentialParameters, val: i32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`, `PublicKeyCredentialType`*"]
#[js::import = "None"]
pub fn get_type(this: &PublicKeyCredentialParameters) -> PublicKeyCredentialType { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`, `PublicKeyCredentialType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &PublicKeyCredentialParameters, val: PublicKeyCredentialType) { panic!("web-sys intrinsic") }
impl PublicKeyCredentialParameters {
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Construct a new `PublicKeyCredentialParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialParameters`, `PublicKeyCredentialType`*"]
#[js::import = "None"]
pub fn new(alg: i32, type_: PublicKeyCredentialType) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_alg(alg);
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_alg()` instead."]
#[js::import = "None"]
pub fn alg(&mut self, val: i32) -> &mut Self { self.set_alg(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: PublicKeyCredentialType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
