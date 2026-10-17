#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UdpMessageEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct UdpMessageEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &UdpMessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &UdpMessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &UdpMessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &UdpMessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &UdpMessageEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &UdpMessageEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None"]
pub fn get_data(this: &UdpMessageEventInit) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_data(this: &UdpMessageEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None"]
pub fn get_remote_address(this: &UdpMessageEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_remote_address(this: &UdpMessageEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None"]
pub fn get_remote_port(this: &UdpMessageEventInit) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
#[js::import = "None = $1"]
pub fn set_remote_port(this: &UdpMessageEventInit, val: u16) { panic!("web-sys intrinsic") }
impl UdpMessageEventInit {
    #[doc = "Construct a new `UdpMessageEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpMessageEventInit`*"]
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
    #[deprecated = "Use `set_data()` instead."]
#[js::import = "None"]
pub fn data(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_data(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_remote_address()` instead."]
#[js::import = "None"]
pub fn remote_address(&mut self, val: &str) -> &mut Self { self.set_remote_address(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_remote_port()` instead."]
#[js::import = "None"]
pub fn remote_port(&mut self, val: u16) -> &mut Self { self.set_remote_port(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for UdpMessageEventInit {
    fn default() -> Self {
        Self::new()
    }
}
