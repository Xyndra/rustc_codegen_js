#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AutocompleteInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AutocompleteInfo(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `addressType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::import = "None"]
pub fn get_address_type(this: &AutocompleteInfo) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `addressType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::import = "None = $1"]
pub fn set_address_type(this: &AutocompleteInfo, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `contactType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::import = "None"]
pub fn get_contact_type(this: &AutocompleteInfo) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `contactType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::import = "None = $1"]
pub fn set_contact_type(this: &AutocompleteInfo, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::import = "None"]
pub fn get_field_name(this: &AutocompleteInfo) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::import = "None = $1"]
pub fn set_field_name(this: &AutocompleteInfo, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `section` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::import = "None"]
pub fn get_section(this: &AutocompleteInfo) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `section` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::import = "None = $1"]
pub fn set_section(this: &AutocompleteInfo, val: &str) { panic!("web-sys intrinsic") }
impl AutocompleteInfo {
    #[doc = "Construct a new `AutocompleteInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_address_type()` instead."]
#[js::import = "None"]
pub fn address_type(&mut self, val: &str) -> &mut Self { self.set_address_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_contact_type()` instead."]
#[js::import = "None"]
pub fn contact_type(&mut self, val: &str) -> &mut Self { self.set_contact_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_field_name()` instead."]
#[js::import = "None"]
pub fn field_name(&mut self, val: &str) -> &mut Self { self.set_field_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_section()` instead."]
#[js::import = "None"]
pub fn section(&mut self, val: &str) -> &mut Self { self.set_section(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AutocompleteInfo {
    fn default() -> Self {
        Self::new()
    }
}
