#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomRectInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomRectInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::import = "None"]
pub fn get_height(this: &DomRectInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::import = "None = $1"]
pub fn set_height(this: &DomRectInit, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::import = "None"]
pub fn get_width(this: &DomRectInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::import = "None = $1"]
pub fn set_width(this: &DomRectInit, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::import = "None"]
pub fn get_x(this: &DomRectInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::import = "None = $1"]
pub fn set_x(this: &DomRectInit, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::import = "None"]
pub fn get_y(this: &DomRectInit) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::import = "None = $1"]
pub fn set_y(this: &DomRectInit, val: f64) { panic!("web-sys intrinsic") }
impl DomRectInit {
    #[doc = "Construct a new `DomRectInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_height()` instead."]
#[js::import = "None"]
pub fn height(&mut self, val: f64) -> &mut Self { self.set_height(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_width()` instead."]
#[js::import = "None"]
pub fn width(&mut self, val: f64) -> &mut Self { self.set_width(val) { panic!("web-sys intrinsic") }
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
}
impl Default for DomRectInit {
    fn default() -> Self {
        Self::new()
    }
}
