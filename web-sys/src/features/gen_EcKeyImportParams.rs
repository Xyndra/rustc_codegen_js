#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcKeyImportParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct EcKeyImportParams(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
#[js::import = "None"]
pub fn get_name(this: &EcKeyImportParams) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &EcKeyImportParams, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
#[js::import = "None"]
pub fn get_named_curve(this: &EcKeyImportParams) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
#[js::import = "None = $1"]
pub fn set_named_curve(this: &EcKeyImportParams, val: &str) { panic!("web-sys intrinsic") }
impl EcKeyImportParams {
    #[doc = "Construct a new `EcKeyImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
#[js::import = "None"]
pub fn new(name: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_name(name);
        ret
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_named_curve()` instead."]
#[js::import = "None"]
pub fn named_curve(&mut self, val: &str) -> &mut Self { self.set_named_curve(val) { panic!("web-sys intrinsic") }
        self
    }
}
