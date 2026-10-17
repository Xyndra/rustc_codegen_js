#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PushEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &PushEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &PushEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &PushEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &PushEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &PushEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &PushEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
#[js::import = "None"]
pub fn get_data(this: &PushEventInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
#[js::import = "None = $1"]
pub fn set_data(this: &PushEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl PushEventInit {
    #[doc = "Construct a new `PushEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushEventInit`*"]
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
    #[deprecated = "Use `set_data()` instead."]
#[js::import = "None"]
pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_data(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for PushEventInit {
    fn default() -> Self {
        Self::new()
    }
}
