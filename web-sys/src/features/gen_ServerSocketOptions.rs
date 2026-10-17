#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServerSocketOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ServerSocketOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Get the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpSocketBinaryType`*"]
#[js::import = "None"]
pub fn get_binary_type(this: &ServerSocketOptions) -> Option<TcpSocketBinaryType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Change the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpSocketBinaryType`*"]
#[js::import = "None = $1"]
pub fn set_binary_type(this: &ServerSocketOptions, val: TcpSocketBinaryType) { panic!("web-sys intrinsic") }
impl ServerSocketOptions {
    #[doc = "Construct a new `ServerSocketOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`*"]
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
}
impl Default for ServerSocketOptions {
    fn default() -> Self {
        Self::new()
    }
}
