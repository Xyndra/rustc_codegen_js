#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NativeOsFileReadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NativeOsFileReadOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
#[js::import = "None"]
pub fn get_bytes(this: &NativeOsFileReadOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
#[js::import = "None = $1"]
pub fn set_bytes(this: &NativeOsFileReadOptions, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
#[js::import = "None"]
pub fn get_encoding(this: &NativeOsFileReadOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
#[js::import = "None = $1"]
pub fn set_encoding(this: &NativeOsFileReadOptions, val: Option<&str>) { panic!("web-sys intrinsic") }
impl NativeOsFileReadOptions {
    #[doc = "Construct a new `NativeOsFileReadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_bytes()` instead."]
#[js::import = "None"]
pub fn bytes(&mut self, val: Option<f64>) -> &mut Self { self.set_bytes(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_encoding()` instead."]
#[js::import = "None"]
pub fn encoding(&mut self, val: Option<&str>) -> &mut Self { self.set_encoding(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for NativeOsFileReadOptions {
    fn default() -> Self {
        Self::new()
    }
}
