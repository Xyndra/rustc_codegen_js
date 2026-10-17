#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbFileMetadataParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    #[deprecated]
#[js::opaque]
#[derive(Copy)]
pub struct IdbFileMetadataParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_last_modified(this: &IdbFileMetadataParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_last_modified(this: &IdbFileMetadataParameters, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_size(this: &IdbFileMetadataParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_size(this: &IdbFileMetadataParameters, val: bool) { panic!("web-sys intrinsic") }
impl IdbFileMetadataParameters {
    #[doc = "Construct a new `IdbFileMetadataParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileMetadataParameters`*"]
    #[deprecated]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_last_modified()` instead."]
#[js::import = "None"]
pub fn last_modified(&mut self, val: bool) -> &mut Self { self.set_last_modified(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_size()` instead."]
#[js::import = "None"]
pub fn size(&mut self, val: bool) -> &mut Self { self.set_size(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for IdbFileMetadataParameters {
    fn default() -> Self {
        Self::new()
    }
}
