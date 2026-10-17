#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainDoubleRange` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConstrainDoubleRange(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::import = "None"]
pub fn get_exact(this: &ConstrainDoubleRange) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::import = "None = $1"]
pub fn set_exact(this: &ConstrainDoubleRange, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::import = "None"]
pub fn get_ideal(this: &ConstrainDoubleRange) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::import = "None = $1"]
pub fn set_ideal(this: &ConstrainDoubleRange, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::import = "None"]
pub fn get_max(this: &ConstrainDoubleRange) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::import = "None = $1"]
pub fn set_max(this: &ConstrainDoubleRange, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::import = "None"]
pub fn get_min(this: &ConstrainDoubleRange) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::import = "None = $1"]
pub fn set_min(this: &ConstrainDoubleRange, val: f64) { panic!("web-sys intrinsic") }
impl ConstrainDoubleRange {
    #[doc = "Construct a new `ConstrainDoubleRange`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDoubleRange`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_exact()` instead."]
#[js::import = "None"]
pub fn exact(&mut self, val: f64) -> &mut Self { self.set_exact(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_ideal()` instead."]
#[js::import = "None"]
pub fn ideal(&mut self, val: f64) -> &mut Self { self.set_ideal(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_max()` instead."]
#[js::import = "None"]
pub fn max(&mut self, val: f64) -> &mut Self { self.set_max(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_min()` instead."]
#[js::import = "None"]
pub fn min(&mut self, val: f64) -> &mut Self { self.set_min(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConstrainDoubleRange {
    fn default() -> Self {
        Self::new()
    }
}
