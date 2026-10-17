#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceObserverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceObserverInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
#[js::import = "None"]
pub fn get_buffered(this: &PerformanceObserverInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `buffered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
#[js::import = "None = $1"]
pub fn set_buffered(this: &PerformanceObserverInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `entryTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
#[js::import = "None"]
pub fn get_entry_types(this: &PerformanceObserverInit) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[doc = "Change the `entryTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
#[js::import = "None = $1"]
pub fn set_entry_types(this: &PerformanceObserverInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl PerformanceObserverInit {
    #[doc = "Construct a new `PerformanceObserverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceObserverInit`*"]
#[js::import = "None"]
pub fn new(entry_types: &::wasm_bindgen::JsValue) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_entry_types(entry_types);
        ret
    }
    #[deprecated = "Use `set_buffered()` instead."]
#[js::import = "None"]
pub fn buffered(&mut self, val: bool) -> &mut Self { self.set_buffered(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_entry_types()` instead."]
#[js::import = "None"]
pub fn entry_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_entry_types(val) { panic!("web-sys intrinsic") }
        self
    }
}
