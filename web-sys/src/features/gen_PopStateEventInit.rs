#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PopStateEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PopStateEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &PopStateEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &PopStateEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &PopStateEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &PopStateEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &PopStateEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &PopStateEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
#[js::import = "None"]
pub fn get_state(this: &PopStateEventInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `state` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
#[js::import = "None = $1"]
pub fn set_state(this: &PopStateEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl PopStateEventInit {
    #[doc = "Construct a new `PopStateEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PopStateEventInit`*"]
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
    #[deprecated = "Use `set_state()` instead."]
#[js::import = "None"]
pub fn state(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_state(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PopStateEventInit {
    fn default() -> Self {
        Self::new()
    }
}
