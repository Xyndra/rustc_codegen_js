#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageEstimate` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StorageEstimate(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `quota` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
#[js::import = "None"]
pub fn get_quota(this: &StorageEstimate) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `quota` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
#[js::import = "None = $1"]
pub fn set_quota(this: &StorageEstimate, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
#[js::import = "None"]
pub fn get_usage(this: &StorageEstimate) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
#[js::import = "None = $1"]
pub fn set_usage(this: &StorageEstimate, val: f64) { panic!("web-sys intrinsic") }
impl StorageEstimate {
    #[doc = "Construct a new `StorageEstimate`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_quota()` instead."]
#[js::import = "None"]
pub fn quota(&mut self, val: f64) -> &mut Self { self.set_quota(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_usage()` instead."]
#[js::import = "None"]
pub fn usage(&mut self, val: f64) -> &mut Self { self.set_usage(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for StorageEstimate {
    fn default() -> Self {
        Self::new()
    }
}
