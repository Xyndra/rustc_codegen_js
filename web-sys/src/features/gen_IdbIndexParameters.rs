#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbIndexParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbIndexParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_locale(this: &IdbIndexParameters) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_locale(this: &IdbIndexParameters, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `multiEntry` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
#[js::import = "None"]
pub fn get_multi_entry(this: &IdbIndexParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `multiEntry` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
#[js::import = "None = $1"]
pub fn set_multi_entry(this: &IdbIndexParameters, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `unique` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
#[js::import = "None"]
pub fn get_unique(this: &IdbIndexParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `unique` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
#[js::import = "None = $1"]
pub fn set_unique(this: &IdbIndexParameters, val: bool) { panic!("web-sys intrinsic") }
impl IdbIndexParameters {
    #[doc = "Construct a new `IdbIndexParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbIndexParameters`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_locale()` instead."]
#[js::import = "None"]
pub fn locale(&mut self, val: Option<&str>) -> &mut Self { self.set_locale(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_multi_entry()` instead."]
#[js::import = "None"]
pub fn multi_entry(&mut self, val: bool) -> &mut Self { self.set_multi_entry(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_unique()` instead."]
#[js::import = "None"]
pub fn unique(&mut self, val: bool) -> &mut Self { self.set_unique(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for IdbIndexParameters {
    fn default() -> Self {
        Self::new()
    }
}
