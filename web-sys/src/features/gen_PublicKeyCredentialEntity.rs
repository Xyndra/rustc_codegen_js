#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialEntity` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PublicKeyCredentialEntity(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_icon(this: &PublicKeyCredentialEntity) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_icon(this: &PublicKeyCredentialEntity, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
#[js::import = "None"]
pub fn get_name(this: &PublicKeyCredentialEntity) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &PublicKeyCredentialEntity, val: &str) { panic!("web-sys intrinsic") }
impl PublicKeyCredentialEntity {
    #[doc = "Construct a new `PublicKeyCredentialEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialEntity`*"]
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
}
