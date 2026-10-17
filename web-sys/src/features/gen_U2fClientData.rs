#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `U2fClientData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
#[js::opaque]
#[derive(Copy)]
pub struct U2fClientData(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
#[js::import = "None"]
pub fn get_challenge(this: &U2fClientData) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
#[js::import = "None = $1"]
pub fn set_challenge(this: &U2fClientData, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
#[js::import = "None"]
pub fn get_origin(this: &U2fClientData) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
#[js::import = "None = $1"]
pub fn set_origin(this: &U2fClientData, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `typ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
#[js::import = "None"]
pub fn get_typ(this: &U2fClientData) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `typ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
#[js::import = "None = $1"]
pub fn set_typ(this: &U2fClientData, val: &str) { panic!("web-sys intrinsic") }
impl U2fClientData {
    #[doc = "Construct a new `U2fClientData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_challenge()` instead."]
#[js::import = "None"]
pub fn challenge(&mut self, val: &str) -> &mut Self { self.set_challenge(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_origin()` instead."]
#[js::import = "None"]
pub fn origin(&mut self, val: &str) -> &mut Self { self.set_origin(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_typ()` instead."]
#[js::import = "None"]
pub fn typ(&mut self, val: &str) -> &mut Self { self.set_typ(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for U2fClientData {
    fn default() -> Self {
        Self::new()
    }
}
