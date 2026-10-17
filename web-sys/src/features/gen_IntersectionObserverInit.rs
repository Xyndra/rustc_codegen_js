#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IntersectionObserverInit(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Element")]
    #[doc = "Get the `root` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverInit`*"]
#[js::import = "None"]
pub fn get_root(this: &IntersectionObserverInit) -> Option<Element> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Element")]
    #[doc = "Change the `root` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverInit`*"]
#[js::import = "None = $1"]
pub fn set_root(this: &IntersectionObserverInit, val: Option<&Element>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `rootMargin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
#[js::import = "None"]
pub fn get_root_margin(this: &IntersectionObserverInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rootMargin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
#[js::import = "None = $1"]
pub fn set_root_margin(this: &IntersectionObserverInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
#[js::import = "None"]
pub fn get_threshold(this: &IntersectionObserverInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
#[js::import = "None = $1"]
pub fn set_threshold(this: &IntersectionObserverInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl IntersectionObserverInit {
    #[doc = "Construct a new `IntersectionObserverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "Element")]
    #[deprecated = "Use `set_root()` instead."]
#[js::import = "None"]
pub fn root(&mut self, val: Option<&Element>) -> &mut Self { self.set_root(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_root_margin()` instead."]
#[js::import = "None"]
pub fn root_margin(&mut self, val: &str) -> &mut Self { self.set_root_margin(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_threshold()` instead."]
#[js::import = "None"]
pub fn threshold(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_threshold(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for IntersectionObserverInit {
    fn default() -> Self {
        Self::new()
    }
}
