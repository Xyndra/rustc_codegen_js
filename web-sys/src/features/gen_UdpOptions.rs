#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UdpOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct UdpOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `addressReuse` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None"]
pub fn get_address_reuse(this: &UdpOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `addressReuse` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None = $1"]
pub fn set_address_reuse(this: &UdpOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `localAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None"]
pub fn get_local_address(this: &UdpOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `localAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None = $1"]
pub fn set_local_address(this: &UdpOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None"]
pub fn get_local_port(this: &UdpOptions) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None = $1"]
pub fn set_local_port(this: &UdpOptions, val: u16) { panic!("web-sys intrinsic") }
    #[doc = "Get the `loopback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None"]
pub fn get_loopback(this: &UdpOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `loopback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None = $1"]
pub fn set_loopback(this: &UdpOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None"]
pub fn get_remote_address(this: &UdpOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None = $1"]
pub fn set_remote_address(this: &UdpOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None"]
pub fn get_remote_port(this: &UdpOptions) -> Option<u16> { panic!("web-sys intrinsic") }
    #[doc = "Change the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None = $1"]
pub fn set_remote_port(this: &UdpOptions, val: u16) { panic!("web-sys intrinsic") }
impl UdpOptions {
    #[doc = "Construct a new `UdpOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_address_reuse()` instead."]
#[js::import = "None"]
pub fn address_reuse(&mut self, val: bool) -> &mut Self { self.set_address_reuse(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_local_address()` instead."]
#[js::import = "None"]
pub fn local_address(&mut self, val: &str) -> &mut Self { self.set_local_address(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_local_port()` instead."]
#[js::import = "None"]
pub fn local_port(&mut self, val: u16) -> &mut Self { self.set_local_port(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_loopback()` instead."]
#[js::import = "None"]
pub fn loopback(&mut self, val: bool) -> &mut Self { self.set_loopback(val) { panic!("web-sys intrinsic") }
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
impl Default for UdpOptions {
    fn default() -> Self {
        Self::new()
    }
}
