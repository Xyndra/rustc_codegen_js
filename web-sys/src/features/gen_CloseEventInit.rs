#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CloseEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CloseEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &CloseEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &CloseEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &CloseEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &CloseEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &CloseEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &CloseEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None"]
pub fn get_code(this: &CloseEventInit) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `code` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_code(this: &CloseEventInit, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None"]
pub fn get_reason(this: &CloseEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_reason(this: &CloseEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `wasClean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None"]
pub fn get_was_clean(this: &CloseEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `wasClean` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
#[js::import = "None = $1"]
pub fn set_was_clean(this: &CloseEventInit, val: bool) { panic!("web-sys intrinsic") }
impl CloseEventInit {
    #[doc = "Construct a new `CloseEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CloseEventInit`*"]
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
    #[deprecated = "Use `set_code()` instead."]
#[js::import = "None"]
pub fn code(&mut self, val: u16) -> &mut Self { self.set_code(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_reason()` instead."]
#[js::import = "None"]
pub fn reason(&mut self, val: &str) -> &mut Self { self.set_reason(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_was_clean()` instead."]
#[js::import = "None"]
pub fn was_clean(&mut self, val: bool) -> &mut Self { self.set_was_clean(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for CloseEventInit {
    fn default() -> Self {
        Self::new()
    }
}
