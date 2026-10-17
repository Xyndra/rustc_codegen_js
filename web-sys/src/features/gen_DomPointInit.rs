#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomPointInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomPointInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `w` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::import = "None"]
pub fn get_w(this: &DomPointInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `w` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::import = "None = $1"]
pub fn set_w(this: &DomPointInit, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::import = "None"]
pub fn get_x(this: &DomPointInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::import = "None = $1"]
pub fn set_x(this: &DomPointInit, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::import = "None"]
pub fn get_y(this: &DomPointInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::import = "None = $1"]
pub fn set_y(this: &DomPointInit, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::import = "None"]
pub fn get_z(this: &DomPointInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `z` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::import = "None = $1"]
pub fn set_z(this: &DomPointInit, val: f64) { panic!("web-sys intrinsic") }
impl DomPointInit {
    #[doc = "Construct a new `DomPointInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_w()` instead."]
#[js::import = "None"]
pub fn w(&mut self, val: f64) -> &mut Self { self.set_w(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_x()` instead."]
#[js::import = "None"]
pub fn x(&mut self, val: f64) -> &mut Self { self.set_x(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_y()` instead."]
#[js::import = "None"]
pub fn y(&mut self, val: f64) -> &mut Self { self.set_y(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_z()` instead."]
#[js::import = "None"]
pub fn z(&mut self, val: f64) -> &mut Self { self.set_z(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DomPointInit {
    fn default() -> Self {
        Self::new()
    }
}
