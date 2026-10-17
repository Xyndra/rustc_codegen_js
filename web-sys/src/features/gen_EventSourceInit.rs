#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventSourceInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct EventSourceInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
#[js::import = "None"]
pub fn get_with_credentials(this: &EventSourceInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `withCredentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
#[js::import = "None = $1"]
pub fn set_with_credentials(this: &EventSourceInit, val: bool) { panic!("web-sys intrinsic") }
impl EventSourceInit {
    #[doc = "Construct a new `EventSourceInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventSourceInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_with_credentials()` instead."]
#[js::import = "None"]
pub fn with_credentials(&mut self, val: bool) -> &mut Self { self.set_with_credentials(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for EventSourceInit {
    fn default() -> Self {
        Self::new()
    }
}
