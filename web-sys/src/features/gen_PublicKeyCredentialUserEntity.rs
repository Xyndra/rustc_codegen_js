#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PublicKeyCredentialUserEntity` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PublicKeyCredentialUserEntity(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_icon(this: &PublicKeyCredentialUserEntity) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `icon` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_icon(this: &PublicKeyCredentialUserEntity, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
#[js::import = "None"]
pub fn get_name(this: &PublicKeyCredentialUserEntity) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &PublicKeyCredentialUserEntity, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
#[js::import = "None"]
pub fn get_display_name(this: &PublicKeyCredentialUserEntity) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `displayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
#[js::import = "None = $1"]
pub fn set_display_name(this: &PublicKeyCredentialUserEntity, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
#[js::import = "None"]
pub fn get_id(this: &PublicKeyCredentialUserEntity) -> ::js_sys::Object { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &PublicKeyCredentialUserEntity, val: &::js_sys::Object) { panic!("web-sys intrinsic") }
impl PublicKeyCredentialUserEntity {
    #[doc = "Construct a new `PublicKeyCredentialUserEntity`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialUserEntity`*"]
#[js::import = "None"]
pub fn new(name: &str, display_name: &str, id: &::js_sys::Object) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret.set_display_name(display_name);
        ret.set_id(id);
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
    #[deprecated = "Use `set_display_name()` instead."]
#[js::import = "None"]
pub fn display_name(&mut self, val: &str) -> &mut Self { self.set_display_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_id()` instead."]
#[js::import = "None"]
pub fn id(&mut self, val: &::js_sys::Object) -> &mut Self { self.set_id(val) { panic!("web-sys intrinsic") }
        self
    }
}
