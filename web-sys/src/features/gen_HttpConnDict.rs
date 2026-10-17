#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HttpConnDict(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `connections` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
#[js::import = "None"]
pub fn get_connections(this: &HttpConnDict) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `connections` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
#[js::import = "None = $1"]
pub fn set_connections(this: &HttpConnDict, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl HttpConnDict {
    #[doc = "Construct a new `HttpConnDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnDict`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_connections()` instead."]
#[js::import = "None"]
pub fn connections(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_connections(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for HttpConnDict {
    fn default() -> Self {
        Self::new()
    }
}
