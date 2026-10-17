#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SocketOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Get the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocketBinaryType`*"]
#[js::import = "None"]
pub fn get_binary_type(this: &SocketOptions) -> Option<TcpSocketBinaryType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Change the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocketBinaryType`*"]
#[js::import = "None = $1"]
pub fn set_binary_type(this: &SocketOptions, val: TcpSocketBinaryType) { panic!("web-sys intrinsic") }
    #[doc = "Get the `useSecureTransport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
#[js::import = "None"]
pub fn get_use_secure_transport(this: &SocketOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `useSecureTransport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
#[js::import = "None = $1"]
pub fn set_use_secure_transport(this: &SocketOptions, val: bool) { panic!("web-sys intrinsic") }
impl SocketOptions {
    #[doc = "Construct a new `SocketOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "TcpSocketBinaryType")]
    #[deprecated = "Use `set_binary_type()` instead."]
#[js::import = "None"]
pub fn binary_type(&mut self, val: TcpSocketBinaryType) -> &mut Self { self.set_binary_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_use_secure_transport()` instead."]
#[js::import = "None"]
pub fn use_secure_transport(&mut self, val: bool) -> &mut Self { self.set_use_secure_transport(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for SocketOptions {
    fn default() -> Self {
        Self::new()
    }
}
