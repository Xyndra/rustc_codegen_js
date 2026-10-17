#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialRpEntity` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PublicKeyCredentialRpEntity(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_icon(this: &PublicKeyCredentialRpEntity) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_icon(this: &PublicKeyCredentialRpEntity, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
#[js::import = "None"]
pub fn get_name(this: &PublicKeyCredentialRpEntity) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &PublicKeyCredentialRpEntity, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
#[js::import = "None"]
pub fn get_id(this: &PublicKeyCredentialRpEntity) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &PublicKeyCredentialRpEntity, val: &str) { panic!("web-sys intrinsic") }
impl PublicKeyCredentialRpEntity {
    #[doc = "Construct a new `PublicKeyCredentialRpEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialRpEntity`*"]
#[js::import = "None"]
pub fn new(name: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret
    }
    #[deprecated = "Use `set_icon()` instead."]
#[js::import = "None"]
pub fn icon(&mut self, val: &str) -> &mut Self { self.set_icon(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_id()` instead."]
#[js::import = "None"]
pub fn id(&mut self, val: &str) -> &mut Self { self.set_id(val) { panic!("web-sys intrinsic") }
        self
    }
}
