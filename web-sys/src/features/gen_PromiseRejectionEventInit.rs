#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PromiseRejectionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PromiseRejectionEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &PromiseRejectionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &PromiseRejectionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &PromiseRejectionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &PromiseRejectionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &PromiseRejectionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &PromiseRejectionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `promise` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None"]
pub fn get_promise(this: &PromiseRejectionEventInit) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
    #[doc = "Change the `promise` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_promise(this: &PromiseRejectionEventInit, val: &::js_sys::Promise) { panic!("web-sys intrinsic") }
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None"]
pub fn get_reason(this: &PromiseRejectionEventInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_reason(this: &PromiseRejectionEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl PromiseRejectionEventInit {
    #[doc = "Construct a new `PromiseRejectionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PromiseRejectionEventInit`*"]
#[js::import = "None"]
pub fn new(promise: &::js_sys::Promise) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_promise(promise);
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
    #[deprecated = "Use `set_promise()` instead."]
#[js::import = "None"]
pub fn promise(&mut self, val: &::js_sys::Promise) -> &mut Self { self.set_promise(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_reason()` instead."]
#[js::import = "None"]
pub fn reason(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_reason(val) { panic!("web-sys intrinsic") }
        self
    }
}
