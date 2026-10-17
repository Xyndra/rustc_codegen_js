#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `L10nElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct L10nElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `l10nArgs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None"]
pub fn get_l10n_args(this: &L10nElement) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `l10nArgs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None = $1"]
pub fn set_l10n_args(this: &L10nElement, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None"]
pub fn get_l10n_attrs(this: &L10nElement) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `l10nAttrs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None = $1"]
pub fn set_l10n_attrs(this: &L10nElement, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None"]
pub fn get_l10n_id(this: &L10nElement) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `l10nId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None = $1"]
pub fn set_l10n_id(this: &L10nElement, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None"]
pub fn get_local_name(this: &L10nElement) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `localName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None = $1"]
pub fn set_local_name(this: &L10nElement, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None"]
pub fn get_namespace_uri(this: &L10nElement) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `namespaceURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None = $1"]
pub fn set_namespace_uri(this: &L10nElement, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None"]
pub fn get_type(this: &L10nElement) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &L10nElement, val: Option<&str>) { panic!("web-sys intrinsic") }
impl L10nElement {
    #[doc = "Construct a new `L10nElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nElement`*"]
#[js::import = "None"]
pub fn new(l10n_id: &str, local_name: &str, namespace_uri: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_l10n_id(l10n_id);
        ret.set_local_name(local_name);
        ret.set_namespace_uri(namespace_uri);
        ret
    }
    #[deprecated = "Use `set_l10n_args()` instead."]
#[js::import = "None"]
pub fn l10n_args(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_l10n_args(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_l10n_attrs()` instead."]
#[js::import = "None"]
pub fn l10n_attrs(&mut self, val: Option<&str>) -> &mut Self { self.set_l10n_attrs(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_l10n_id()` instead."]
#[js::import = "None"]
pub fn l10n_id(&mut self, val: &str) -> &mut Self { self.set_l10n_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_local_name()` instead."]
#[js::import = "None"]
pub fn local_name(&mut self, val: &str) -> &mut Self { self.set_local_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_namespace_uri()` instead."]
#[js::import = "None"]
pub fn namespace_uri(&mut self, val: &str) -> &mut Self { self.set_namespace_uri(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: Option<&str>) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
