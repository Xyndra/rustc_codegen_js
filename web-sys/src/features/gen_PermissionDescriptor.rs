#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PermissionDescriptor` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PermissionDescriptor(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "PermissionName")]
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`, `PermissionName`*"]
#[js::import = "None"]
pub fn get_name(this: &PermissionDescriptor) -> PermissionName { panic!("web-sys intrinsic") }
    #[cfg(feature = "PermissionName")]
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`, `PermissionName`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &PermissionDescriptor, val: PermissionName) { panic!("web-sys intrinsic") }
impl PermissionDescriptor {
    #[cfg(feature = "PermissionName")]
    #[doc = "Construct a new `PermissionDescriptor`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PermissionDescriptor`, `PermissionName`*"]
#[js::import = "None"]
pub fn new(name: PermissionName) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret
    }
    #[cfg(feature = "PermissionName")]
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: PermissionName) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
}
