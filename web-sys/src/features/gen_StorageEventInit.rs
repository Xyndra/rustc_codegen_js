#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StorageEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &StorageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &StorageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &StorageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &StorageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &StorageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &StorageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None"]
pub fn get_key(this: &StorageEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_key(this: &StorageEventInit, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None"]
pub fn get_new_value(this: &StorageEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_new_value(this: &StorageEventInit, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None"]
pub fn get_old_value(this: &StorageEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `oldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_old_value(this: &StorageEventInit, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Storage")]
    #[doc = "Get the `storageArea` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEventInit`*"]
#[js::import = "None"]
pub fn get_storage_area(this: &StorageEventInit) -> Option<Storage> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Storage")]
    #[doc = "Change the `storageArea` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `StorageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_storage_area(this: &StorageEventInit, val: Option<&Storage>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None"]
pub fn get_url(this: &StorageEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `url` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_url(this: &StorageEventInit, val: &str) { panic!("web-sys intrinsic") }
impl StorageEventInit {
    #[doc = "Construct a new `StorageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_key()` instead."]
#[js::import = "None"]
pub fn key(&mut self, val: Option<&str>) -> &mut Self { self.set_key(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_new_value()` instead."]
#[js::import = "None"]
pub fn new_value(&mut self, val: Option<&str>) -> &mut Self { self.set_new_value(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_old_value()` instead."]
#[js::import = "None"]
pub fn old_value(&mut self, val: Option<&str>) -> &mut Self { self.set_old_value(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "Storage")]
    #[deprecated = "Use `set_storage_area()` instead."]
#[js::import = "None"]
pub fn storage_area(&mut self, val: Option<&Storage>) -> &mut Self { self.set_storage_area(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_url()` instead."]
#[js::import = "None"]
pub fn url(&mut self, val: &str) -> &mut Self { self.set_url(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for StorageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
