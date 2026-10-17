#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FocusEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FocusEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &FocusEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &FocusEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &FocusEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &FocusEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &FocusEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &FocusEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
#[js::import = "None"]
pub fn get_detail(this: &FocusEventInit) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
#[js::import = "None = $1"]
pub fn set_detail(this: &FocusEventInit, val: i32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`, `Window`*"]
#[js::import = "None"]
pub fn get_view(this: &FocusEventInit) -> Option<Window> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`, `Window`*"]
#[js::import = "None = $1"]
pub fn set_view(this: &FocusEventInit, val: Option<&Window>) { panic!("web-sys intrinsic") }
    #[cfg(feature = "EventTarget")]
    #[doc = "Get the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `FocusEventInit`*"]
#[js::import = "None"]
pub fn get_related_target(this: &FocusEventInit) -> Option<EventTarget> { panic!("web-sys intrinsic") }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `relatedTarget` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `FocusEventInit`*"]
#[js::import = "None = $1"]
pub fn set_related_target(this: &FocusEventInit, val: Option<&EventTarget>) { panic!("web-sys intrinsic") }
impl FocusEventInit {
    #[doc = "Construct a new `FocusEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FocusEventInit`*"]
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
pub fn detail(&mut self, val: i32) -> &mut Self { self.set_detail(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "Window")]
    #[deprecated = "Use `set_view()` instead."]
#[js::import = "None"]
pub fn view(&mut self, val: Option<&Window>) -> &mut Self { self.set_view(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "EventTarget")]
    #[deprecated = "Use `set_related_target()` instead."]
#[js::import = "None"]
pub fn related_target(&mut self, val: Option<&EventTarget>) -> &mut Self { self.set_related_target(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for FocusEventInit {
    fn default() -> Self {
        Self::new()
    }
}
