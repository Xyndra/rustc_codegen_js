#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HttpConnInfo(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `protocolVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
#[js::import = "None"]
pub fn get_protocol_version(this: &HttpConnInfo) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `protocolVersion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
#[js::import = "None = $1"]
pub fn set_protocol_version(this: &HttpConnInfo, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `rtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
#[js::import = "None"]
pub fn get_rtt(this: &HttpConnInfo) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rtt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
#[js::import = "None = $1"]
pub fn set_rtt(this: &HttpConnInfo, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ttl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
#[js::import = "None"]
pub fn get_ttl(this: &HttpConnInfo) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ttl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
#[js::import = "None = $1"]
pub fn set_ttl(this: &HttpConnInfo, val: u32) { panic!("web-sys intrinsic") }
impl HttpConnInfo {
    #[doc = "Construct a new `HttpConnInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnInfo`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_protocol_version()` instead."]
#[js::import = "None"]
pub fn protocol_version(&mut self, val: &str) -> &mut Self { self.set_protocol_version(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_rtt()` instead."]
#[js::import = "None"]
pub fn rtt(&mut self, val: u32) -> &mut Self { self.set_rtt(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ttl()` instead."]
#[js::import = "None"]
pub fn ttl(&mut self, val: u32) -> &mut Self { self.set_ttl(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for HttpConnInfo {
    fn default() -> Self {
        Self::new()
    }
}
