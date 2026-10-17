#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebSocketDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebSocketDict(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `websockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
#[js::import = "None"]
pub fn get_websockets(this: &WebSocketDict) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `websockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
#[js::import = "None = $1"]
pub fn set_websockets(this: &WebSocketDict, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl WebSocketDict {
    #[doc = "Construct a new `WebSocketDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_websockets()` instead."]
#[js::import = "None"]
pub fn websockets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_websockets(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for WebSocketDict {
    fn default() -> Self {
        Self::new()
    }
}
