#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CustomEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CustomEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &CustomEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &CustomEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &CustomEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &CustomEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &CustomEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &CustomEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
#[js::import = "None"]
pub fn get_detail(this: &CustomEventInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
#[js::import = "None = $1"]
pub fn set_detail(this: &CustomEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl CustomEventInit {
    #[doc = "Construct a new `CustomEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomEventInit`*"]
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
    #[deprecated = "Use `set_detail()` instead."]
#[js::import = "None"]
pub fn detail(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_detail(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for CustomEventInit {
    fn default() -> Self {
        Self::new()
    }
}
