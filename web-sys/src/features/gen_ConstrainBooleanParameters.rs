#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainBooleanParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConstrainBooleanParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
#[js::import = "None"]
pub fn get_exact(this: &ConstrainBooleanParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
#[js::import = "None = $1"]
pub fn set_exact(this: &ConstrainBooleanParameters, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
#[js::import = "None"]
pub fn get_ideal(this: &ConstrainBooleanParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
#[js::import = "None = $1"]
pub fn set_ideal(this: &ConstrainBooleanParameters, val: bool) { panic!("web-sys intrinsic") }
impl ConstrainBooleanParameters {
    #[doc = "Construct a new `ConstrainBooleanParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainBooleanParameters`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_exact()` instead."]
#[js::import = "None"]
pub fn exact(&mut self, val: bool) -> &mut Self { self.set_exact(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ideal()` instead."]
#[js::import = "None"]
pub fn ideal(&mut self, val: bool) -> &mut Self { self.set_ideal(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConstrainBooleanParameters {
    fn default() -> Self {
        Self::new()
    }
}
