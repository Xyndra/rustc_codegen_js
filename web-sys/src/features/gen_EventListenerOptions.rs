#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventListenerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct EventListenerOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
#[js::import = "None"]
pub fn get_capture(this: &EventListenerOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
#[js::import = "None = $1"]
pub fn set_capture(this: &EventListenerOptions, val: bool) { panic!("web-sys intrinsic") }
impl EventListenerOptions {
    #[doc = "Construct a new `EventListenerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_capture()` instead."]
#[js::import = "None"]
pub fn capture(&mut self, val: bool) -> &mut Self { self.set_capture(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for EventListenerOptions {
    fn default() -> Self {
        Self::new()
    }
}
