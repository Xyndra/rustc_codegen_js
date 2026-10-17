#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketsDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SocketsDict(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
#[js::import = "None"]
pub fn get_received(this: &SocketsDict) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
#[js::import = "None = $1"]
pub fn set_received(this: &SocketsDict, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
#[js::import = "None"]
pub fn get_sent(this: &SocketsDict) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
#[js::import = "None = $1"]
pub fn set_sent(this: &SocketsDict, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
#[js::import = "None"]
pub fn get_sockets(this: &SocketsDict) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
#[js::import = "None = $1"]
pub fn set_sockets(this: &SocketsDict, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl SocketsDict {
    #[doc = "Construct a new `SocketsDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_received()` instead."]
#[js::import = "None"]
pub fn received(&mut self, val: f64) -> &mut Self { self.set_received(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_sent()` instead."]
#[js::import = "None"]
pub fn sent(&mut self, val: f64) -> &mut Self { self.set_sent(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_sockets()` instead."]
#[js::import = "None"]
pub fn sockets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_sockets(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for SocketsDict {
    fn default() -> Self {
        Self::new()
    }
}
