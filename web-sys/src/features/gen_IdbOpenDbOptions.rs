#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbOpenDbOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
#[js::opaque]
#[derive(Copy)]
pub struct IdbOpenDbOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "StorageType")]
    #[doc = "Get the `storage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`, `StorageType`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_storage(this: &IdbOpenDbOptions) -> Option<StorageType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "StorageType")]
    #[doc = "Change the `storage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`, `StorageType`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_storage(this: &IdbOpenDbOptions, val: StorageType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
#[js::import = "None"]
pub fn get_version(this: &IdbOpenDbOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
#[js::import = "None = $1"]
pub fn set_version(this: &IdbOpenDbOptions, val: f64) { panic!("web-sys intrinsic") }
impl IdbOpenDbOptions {
    #[doc = "Construct a new `IdbOpenDbOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbOpenDbOptions`*"]
    #[deprecated]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "StorageType")]
    #[deprecated = "Use `set_storage()` instead."]
#[js::import = "None"]
pub fn storage(&mut self, val: StorageType) -> &mut Self { self.set_storage(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_version()` instead."]
#[js::import = "None"]
pub fn version(&mut self, val: f64) -> &mut Self { self.set_version(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for IdbOpenDbOptions {
    fn default() -> Self {
        Self::new()
    }
}
