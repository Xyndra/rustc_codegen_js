#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SocketElement(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None"]
pub fn get_active(this: &SocketElement) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None = $1"]
pub fn set_active(this: &SocketElement, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None"]
pub fn get_host(this: &SocketElement) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None = $1"]
pub fn set_host(this: &SocketElement, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None"]
pub fn get_port(this: &SocketElement) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None = $1"]
pub fn set_port(this: &SocketElement, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None"]
pub fn get_received(this: &SocketElement) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None = $1"]
pub fn set_received(this: &SocketElement, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None"]
pub fn get_sent(this: &SocketElement) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None = $1"]
pub fn set_sent(this: &SocketElement, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None"]
pub fn get_tcp(this: &SocketElement) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None = $1"]
pub fn set_tcp(this: &SocketElement, val: bool) { panic!("web-sys intrinsic") }
impl SocketElement {
    #[doc = "Construct a new `SocketElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_active()` instead."]
#[js::import = "None"]
pub fn active(&mut self, val: bool) -> &mut Self { self.set_active(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_host()` instead."]
#[js::import = "None"]
pub fn host(&mut self, val: &str) -> &mut Self { self.set_host(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_port()` instead."]
#[js::import = "None"]
pub fn port(&mut self, val: u32) -> &mut Self { self.set_port(val) { panic!("web-sys intrinsic") }
        self
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
    #[deprecated = "Use `set_tcp()` instead."]
#[js::import = "None"]
pub fn tcp(&mut self, val: bool) -> &mut Self { self.set_tcp(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for SocketElement {
    fn default() -> Self {
        Self::new()
    }
}
