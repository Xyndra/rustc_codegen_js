#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebSocketElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebSocketElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None"]
pub fn get_encrypted(this: &WebSocketElement) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `encrypted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None = $1"]
pub fn set_encrypted(this: &WebSocketElement, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `hostport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None"]
pub fn get_hostport(this: &WebSocketElement) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `hostport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None = $1"]
pub fn set_hostport(this: &WebSocketElement, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `msgreceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None"]
pub fn get_msgreceived(this: &WebSocketElement) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `msgreceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None = $1"]
pub fn set_msgreceived(this: &WebSocketElement, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `msgsent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None"]
pub fn get_msgsent(this: &WebSocketElement) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `msgsent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None = $1"]
pub fn set_msgsent(this: &WebSocketElement, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `receivedsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None"]
pub fn get_receivedsize(this: &WebSocketElement) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `receivedsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None = $1"]
pub fn set_receivedsize(this: &WebSocketElement, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sentsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None"]
pub fn get_sentsize(this: &WebSocketElement) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sentsize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None = $1"]
pub fn set_sentsize(this: &WebSocketElement, val: f64) { panic!("web-sys intrinsic") }
impl WebSocketElement {
    #[doc = "Construct a new `WebSocketElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketElement`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_encrypted()` instead."]
#[js::import = "None"]
pub fn encrypted(&mut self, val: bool) -> &mut Self { self.set_encrypted(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_hostport()` instead."]
#[js::import = "None"]
pub fn hostport(&mut self, val: &str) -> &mut Self { self.set_hostport(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_msgreceived()` instead."]
#[js::import = "None"]
pub fn msgreceived(&mut self, val: u32) -> &mut Self { self.set_msgreceived(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_msgsent()` instead."]
#[js::import = "None"]
pub fn msgsent(&mut self, val: u32) -> &mut Self { self.set_msgsent(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_receivedsize()` instead."]
#[js::import = "None"]
pub fn receivedsize(&mut self, val: f64) -> &mut Self { self.set_receivedsize(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_sentsize()` instead."]
#[js::import = "None"]
pub fn sentsize(&mut self, val: f64) -> &mut Self { self.set_sentsize(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for WebSocketElement {
    fn default() -> Self {
        Self::new()
    }
}
