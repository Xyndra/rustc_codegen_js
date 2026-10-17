#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TransitionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TransitionEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &TransitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &TransitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &TransitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &TransitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &TransitionEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &TransitionEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None"]
pub fn get_elapsed_time(this: &TransitionEventInit) -> Option<f32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `elapsedTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_elapsed_time(this: &TransitionEventInit, val: f32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `propertyName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None"]
pub fn get_property_name(this: &TransitionEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `propertyName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_property_name(this: &TransitionEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None"]
pub fn get_pseudo_element(this: &TransitionEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `pseudoElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
#[js::import = "None = $1"]
pub fn set_pseudo_element(this: &TransitionEventInit, val: &str) { panic!("web-sys intrinsic") }
impl TransitionEventInit {
    #[doc = "Construct a new `TransitionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TransitionEventInit`*"]
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
    #[deprecated = "Use `set_elapsed_time()` instead."]
#[js::import = "None"]
pub fn elapsed_time(&mut self, val: f32) -> &mut Self { self.set_elapsed_time(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_property_name()` instead."]
#[js::import = "None"]
pub fn property_name(&mut self, val: &str) -> &mut Self { self.set_property_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_pseudo_element()` instead."]
#[js::import = "None"]
pub fn pseudo_element(&mut self, val: &str) -> &mut Self { self.set_pseudo_element(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for TransitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
