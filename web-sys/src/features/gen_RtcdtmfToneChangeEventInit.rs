#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcdtmfToneChangeEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcdtmfToneChangeEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &RtcdtmfToneChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &RtcdtmfToneChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &RtcdtmfToneChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &RtcdtmfToneChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &RtcdtmfToneChangeEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &RtcdtmfToneChangeEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `tone` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
#[js::import = "None"]
pub fn get_tone(this: &RtcdtmfToneChangeEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `tone` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
#[js::import = "None = $1"]
pub fn set_tone(this: &RtcdtmfToneChangeEventInit, val: &str) { panic!("web-sys intrinsic") }
impl RtcdtmfToneChangeEventInit {
    #[doc = "Construct a new `RtcdtmfToneChangeEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcdtmfToneChangeEventInit`*"]
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
    #[deprecated = "Use `set_tone()` instead."]
#[js::import = "None"]
pub fn tone(&mut self, val: &str) -> &mut Self { self.set_tone(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RtcdtmfToneChangeEventInit {
    fn default() -> Self {
        Self::new()
    }
}
