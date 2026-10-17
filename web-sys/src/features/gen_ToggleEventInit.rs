#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ToggleEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ToggleEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &ToggleEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &ToggleEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &ToggleEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &ToggleEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &ToggleEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &ToggleEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `newState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None"]
pub fn get_new_state(this: &ToggleEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `newState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None = $1"]
pub fn set_new_state(this: &ToggleEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `oldState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None"]
pub fn get_old_state(this: &ToggleEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `oldState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
#[js::import = "None = $1"]
pub fn set_old_state(this: &ToggleEventInit, val: &str) { panic!("web-sys intrinsic") }
impl ToggleEventInit {
    #[doc = "Construct a new `ToggleEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ToggleEventInit`*"]
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
    #[deprecated = "Use `set_new_state()` instead."]
#[js::import = "None"]
pub fn new_state(&mut self, val: &str) -> &mut Self { self.set_new_state(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_old_state()` instead."]
#[js::import = "None"]
pub fn old_state(&mut self, val: &str) -> &mut Self { self.set_old_state(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ToggleEventInit {
    fn default() -> Self {
        Self::new()
    }
}
