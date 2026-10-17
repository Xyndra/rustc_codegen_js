#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsCacheEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DnsCacheEntry(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None"]
pub fn get_expiration(this: &DnsCacheEntry) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None = $1"]
pub fn set_expiration(this: &DnsCacheEntry, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `family` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None"]
pub fn get_family(this: &DnsCacheEntry) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `family` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None = $1"]
pub fn set_family(this: &DnsCacheEntry, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `hostaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None"]
pub fn get_hostaddr(this: &DnsCacheEntry) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `hostaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None = $1"]
pub fn set_hostaddr(this: &DnsCacheEntry, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None"]
pub fn get_hostname(this: &DnsCacheEntry) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None = $1"]
pub fn set_hostname(this: &DnsCacheEntry, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `trr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None"]
pub fn get_trr(this: &DnsCacheEntry) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `trr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None = $1"]
pub fn set_trr(this: &DnsCacheEntry, val: bool) { panic!("web-sys intrinsic") }
impl DnsCacheEntry {
    #[doc = "Construct a new `DnsCacheEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_expiration()` instead."]
#[js::import = "None"]
pub fn expiration(&mut self, val: f64) -> &mut Self { self.set_expiration(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_family()` instead."]
#[js::import = "None"]
pub fn family(&mut self, val: &str) -> &mut Self { self.set_family(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_hostaddr()` instead."]
#[js::import = "None"]
pub fn hostaddr(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_hostaddr(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_hostname()` instead."]
#[js::import = "None"]
pub fn hostname(&mut self, val: &str) -> &mut Self { self.set_hostname(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_trr()` instead."]
#[js::import = "None"]
pub fn trr(&mut self, val: bool) -> &mut Self { self.set_trr(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DnsCacheEntry {
    fn default() -> Self {
        Self::new()
    }
}
