#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AddEventListenerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AddEventListenerOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
#[js::import = "None"]
pub fn get_capture(this: &AddEventListenerOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
#[js::import = "None = $1"]
pub fn set_capture(this: &AddEventListenerOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `once` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
#[js::import = "None"]
pub fn get_once(this: &AddEventListenerOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `once` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
#[js::import = "None = $1"]
pub fn set_once(this: &AddEventListenerOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `passive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
#[js::import = "None"]
pub fn get_passive(this: &AddEventListenerOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `passive` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
#[js::import = "None = $1"]
pub fn set_passive(this: &AddEventListenerOptions, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `AddEventListenerOptions`*"]
#[js::import = "None"]
pub fn get_signal(this: &AddEventListenerOptions) -> Option<AbortSignal> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `AddEventListenerOptions`*"]
#[js::import = "None = $1"]
pub fn set_signal(this: &AddEventListenerOptions, val: &AbortSignal) { panic!("web-sys intrinsic") }
impl AddEventListenerOptions {
    #[doc = "Construct a new `AddEventListenerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AddEventListenerOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_capture()` instead."]
#[js::import = "None"]
pub fn capture(&mut self, val: bool) -> &mut Self { self.set_capture(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_once()` instead."]
#[js::import = "None"]
pub fn once(&mut self, val: bool) -> &mut Self { self.set_once(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_passive()` instead."]
#[js::import = "None"]
pub fn passive(&mut self, val: bool) -> &mut Self { self.set_passive(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[deprecated = "Use `set_signal()` instead."]
#[js::import = "None"]
pub fn signal(&mut self, val: &AbortSignal) -> &mut Self { self.set_signal(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for AddEventListenerOptions {
    fn default() -> Self {
        Self::new()
    }
}
