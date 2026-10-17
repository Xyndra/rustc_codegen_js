#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PublicKeyCredentialDescriptor(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
#[js::import = "None"]
pub fn get_id(this: &PublicKeyCredentialDescriptor) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &PublicKeyCredentialDescriptor, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
    #[doc = "Get the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
#[js::import = "None"]
pub fn get_transports(this: &PublicKeyCredentialDescriptor) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `transports` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`*"]
#[js::import = "None = $1"]
pub fn set_transports(this: &PublicKeyCredentialDescriptor, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
#[js::import = "None"]
pub fn get_type(this: &PublicKeyCredentialDescriptor) -> PublicKeyCredentialType { panic!("web-sys intrinsic") }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &PublicKeyCredentialDescriptor, val: PublicKeyCredentialType) { panic!("web-sys intrinsic") }
impl PublicKeyCredentialDescriptor {
    #[cfg(feature = "PublicKeyCredentialType")]
    #[doc = "Construct a new `PublicKeyCredentialDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialDescriptor`, `PublicKeyCredentialType`*"]
#[js::import = "None"]
pub fn new(id: &::js_sys::Object, type_: PublicKeyCredentialType) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_id(id);
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_id()` instead."]
#[js::import = "None"]
pub fn id(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_transports()` instead."]
#[js::import = "None"]
pub fn transports(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_transports(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "PublicKeyCredentialType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: PublicKeyCredentialType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
